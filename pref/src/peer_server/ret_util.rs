
use crate::peer_server::peer;
use std::{fs, net::{self, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream}};
use configparser::ini::Ini;
use crate::core;

pub fn gen_config(
    c_type: &peer::PeerCapability,
    log_level: u32, 
    auth_pass: String, 
    ipv6_addr: Option<Ipv6Addr>) -> Result<(), String> {
    // Get needed info; c type, available socket interfaces. assumes a tcp interface is connected to internet.

    // TODO: ini parser mangled the config after writing, so string replace has to be done
    let config_r = fs::read_to_string("reticulum_dummy_config.conf");
    if let Err(err) = config_r {
        return Err(err.to_string());
    }

    let mut config = config_r.unwrap();

//     //config.load("reticulum_dummy_config.conf")?;

    match c_type {
        peer::PeerCapability::Desktop => {
            config = config.replace("enable_transport_pref-value", "No");
            config = config.replace("share_instance_pref-value", "No");
            
            // config.set("reticulum", "enable_transport", Some("No".to_owned()));
            // config.set("reticulum", "share_instance", Some("No".to_owned()));
        }

        peer::PeerCapability::Server | peer::PeerCapability::PtpRelay => {
            // config.set("reticulum", "enable_transport", Some("Yes".to_owned()));
            config = config.replace("enable_transport_pref-value", "Yes");
        }

        _ => {}
    }

    // TODO: when loglevel is actually implemented for prefengine, make the prefengine loglevel match reticulum's tiers
    // config.set("logging", "loglevel", Some(log_level.to_string()));
    config = config.replace("loglevel_pref-value", &log_level.to_string());

    // TODO: reticulum only supports hardcoded auth passphrases on file, later on this needs to be env var and not just based 
    // off a file
    // config.set("TCP Client Interface", "passphrase", Some(auth_pass));
    config = config.replace("passphrase_pref-value", &auth_pass);

    match ipv6_addr {
        Some(addr) => {
            // config.set("TCP Client Interface", "listen_ip", Some(addr.to_string()));
            // config.set("TCP Client Interface", "target_host", Some(addr.to_string()));   
            config = config.replace("target_host_pref-value", &addr.to_string());
            config = config.replace("listen_ip_pref-value", &addr.to_string()); 
        }
        None => {
            config = config.replace("target_host_pref-value", "0.0.0.0");
            config = config.replace("listen_ip_pref-value", "0.0.0.0"); 
        }
    }

    let path_r = core::dir::get_global_data_path(true);
    if let Err(err) = path_r {
        return Err(err);
    }

    let mut path = path_r.unwrap();
    path.push("config");
    
    match fs::write(path, config) {
        Ok(()) => Ok(()),
        Err(err) => {
            Err(err.to_string())
        }
    }
}
