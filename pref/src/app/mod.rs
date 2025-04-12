use crate::core::dir;
use crate::peer_server::peer::{PeerCapability, RemotePeerInfo, SelfPeerInfo, TempPeerInfo};
use crate::{core, peer_server};

use crate::discovery;
use crate::discovery::NATConfig;
use peer_server::PeerStore;
use peer_server::ret_util;
use std::env;
use std::fs::{self, File};
use std::net::Ipv4Addr;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Application {
    nat: NATConfig,
    client: peer_server::Client,
    online_peers: Arc<Mutex<PeerStore>>,
    ret_process: Child,
}

impl Application {
    pub async fn new() -> Application {
        core::pref_log::init_styled_logger();
        log::info!("Initialized log");

        let nat_conf = NATConfig::new();

        if nat_conf.auto_port_forward {
            log::info!("Auto port-forward enabled");
        } else {
            // TODO: add some sort of message to confirm manual portforwarding
            log::info!("Auto port-forward failed, assuming manual portforward has been done");
        }

        let first_start = Application::is_first_time();

        // first init self peer values
        let capability: PeerCapability = PeerCapability::Desktop;
        let ip_fetch = discovery::get_public_ip()
            .map_err(|e| panic!("Fetching public IP failed: {}", e))
            .unwrap();

        let ip_parse_r = Ipv4Addr::from_str(&ip_fetch);
        if let Err(err) = ip_parse_r {
            panic!("Parsing public IP failed: {}", err);
        }

        let ip_fetch = ip_parse_r.unwrap();
        if first_start {
            let ret_com = {
                if cfg!(target_os = "linux") {
                    Command::new("python3")
                } else {
                    Command::new("python")
                }
            };

            // first_start_ret will use application home path
            match core::dir::get_global_data_path(true) {
                Ok(path) => match fs::create_dir_all(path) {
                    _ => {}
                    Err(e) => panic!("Failed to create app home dir {}", e),
                },
                Err(e) => panic!("Failed to get app home dir: {}", e),
            }

            match Application::first_start_ret(&capability, ret_com) {
                Ok(ret_output) => {
                    // make self peer now that we have our destination hash from python as well
                    SelfPeerInfo::new_self_peer(capability, ip_fetch, ret_output)
                        .map_err(|e| panic!("Creating self peer failed: {}", e));
                }
                Err(e) => panic!("Failed to first start reticulum: {}", e),
            }
        }

        let ret_proc: Child;
        let ret_com = {
            if cfg!(target_os = "linux") {
                Command::new("python3")
            } else {
                Command::new("python")
            }
        };
        match Application::start_pyret(ret_com) {
            Ok(ret_output) => {
                ret_proc = ret_output;
            }
            Err(e) => panic!("Failed to start reticulum: {}", e),
        }

        // ret_proc.kill();

        if let Err(err) = peer_server::db::init() {
            panic!("Starting database failed: {}", err);
        }

        let ps: Arc<Mutex<PeerStore>> = Arc::new(Mutex::new(PeerStore::new()));

        let mut client_inst = peer_server::Client::new(&ps)
            .map_err(|e| panic!("Creating client failed: {}", e))
            .unwrap();

        client_inst
            .start()
            .map_err(|e| panic!("Starting client failed: {}", e));

        use std::thread;
        let listen_inst = peer_server::Listener::new(&ps);
        thread::spawn(move || {
            listen_inst.start();
        });

        return Application {
            nat: nat_conf,
            ret_process: ret_proc,
            online_peers: ps,
            client: client_inst,
        };
    }

    fn group_checks() -> Result<(), String> {
        // check peer group invariants

        // check temp peer invariants

        // check satelite peer invariants
        Ok(())
    }

    fn first_start_ret(
        capability: &PeerCapability,
        mut ret_com: Command,
    ) -> Result<String, String> {
        Application::gen_ret_config(capability)?;

        let ret_path_buf = dir::get_root_file_path("retapi.py")?;
        let ret_path_o = ret_path_buf.to_str();
        if let None = ret_path_o {
            return Err("Root path not valid unicode".to_owned());
        }
        let ret_path = ret_path_o.unwrap();

        let pylog = File::create("pyret.log").map_err(|err| err.to_string())?;
        let stdio = Stdio::from(pylog);

        let hash = ret_com
            .stdout(stdio)
            .args([ret_path, "first_start"])
            .output()
            .map_err(|err| err.to_string())?
            .stdout;

        Ok(String::from_utf8(hash).map_err(|err| err.to_string())?)
    }

    fn start_pyret(mut ret_com: Command) -> Result<Child, String> {
        let ret_path_buf = dir::get_root_file_path("retapi.py")?;
        let ret_path_o = ret_path_buf.to_str();
        if let None = ret_path_o {
            return Err("Root path not valid unicode".to_owned());
        }
        let ret_path = ret_path_o.unwrap();

        let pylog = File::open("pyret.log").map_err(|err| err.to_string())?;
        let stdio = Stdio::from(pylog);

        let ret = ret_com
            .stdout(stdio)
            .args([ret_path])
            .spawn()
            .map_err(|err| err.to_string())?;

        Ok(ret)
    }

    fn gen_ret_config(capability: &PeerCapability) -> Result<(), String> {
        // TODO: reticulum authentication
        let auth_pass = "test_password".to_owned();

        // TODO: support ipv6
        ret_util::gen_config(capability, 6, auth_pass, None)
    }

    pub fn stop() {
        let discov_res = discovery::rmv_upnp_setup();

        match discov_res {
            Ok(()) => {
                log::info!("UPnP port disabled");
            }
            Err(msg) => {
                log::error!("Failed to disable UPnP port: {}", msg);
            }
        }
    }

    pub fn get_db_data(&self) -> Result<String, String> {
        return peer_server::db::db_to_str();
    }

    pub fn update_db(&mut self, rows: String) -> Result<(), String> {
        return peer_server::db::process_local_change(rows, &mut self.client);
    }

    pub fn add_temp_peer(&self, hash: String) -> Result<(), String> {
        // TODO: confirm with rest of peer group
        let temp = TempPeerInfo { dest_hash: hash };
        TempPeerInfo::append_temp_to_disk(temp)?;
        Ok(())
    }

    pub fn all_temp_peers_to_peer(&mut self) -> Result<(), String> {
        // TODO: send this event to rest of peer group
        peer_server::peer::add_all_temp_peers()?;

        // refresh online peers
        self.client.peer_connect_all()?;
        Ok(())
    }

    pub fn get_self_peer(&self) -> Result<SelfPeerInfo, String> {
        SelfPeerInfo::load_self_peer()
    }

    fn is_first_time() -> bool {
        // TODO: improve this, maybe change to looking for config file
        let mut path = env::current_dir().expect("Unable to read current working directory");
        path.push("DO_NOT_DELETE_OR_MOVE");

        if Path::new(&path).exists() {
            return false;
        } else {
            fs::File::create(path)
                .expect("Unable to write 'first start file' to current working directory");
            return true;
        }
    }
}
