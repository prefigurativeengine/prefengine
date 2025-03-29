
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

    // TODO: config parser mangled the config after writing, so string replace has to be done
    let mut config = fs::read_to_string("reticulum_dummy_config.conf")
        .map_err(|err| err.to_string())?;

    match c_type {
        peer::PeerCapability::Desktop => {
            config = config.replace("enable_transport_pref-value", "No");
            config = config.replace("share_instance_pref-value", "No");    
        }

        peer::PeerCapability::Server | peer::PeerCapability::PtpRelay => {
            config = config.replace("enable_transport_pref-value", "Yes");
        }

        _ => {}
    }

    // TODO: when loglevel is actually implemented for prefengine, make the prefengine loglevel match reticulum's tiers
    config = config.replace("loglevel_pref-value", &log_level.to_string());

    // TODO: reticulum only supports hardcoded auth passphrases on file, later on this needs to be env var and not just based 
    // off a file
    config = config.replace("passphrase_pref-value", &auth_pass);

    match ipv6_addr {
        Some(addr) => {
            config = config.replace("target_host_pref-value", &addr.to_string());
            config = config.replace("listen_ip_pref-value", &addr.to_string()); 
        }
        None => {
            config = config.replace("target_host_pref-value", "0.0.0.0");
            config = config.replace("listen_ip_pref-value", "0.0.0.0"); 
        }
    }

    let mut path = core::dir::get_global_data_path(true)?;
    path.push("config");
    
    match fs::write(path, config) {
        Ok(()) => Ok(()),
        Err(err) => {
            Err(err.to_string())
        }
    }
}
