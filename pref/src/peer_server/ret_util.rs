
use crate::peer_server::peer;
use std::net::{self, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream};
use configparser::ini::Ini;

pub fn gen_config(
    c_type: peer::PeerCapability, 
    bt: bool, 
    log_level: u32, 
    auth_pass: String, 
    ipv6_addr: Option<Ipv6Addr>) -> Result<(), String> {
    // Get needed info; c type, available socket interfaces. assumes a tcp interface is connected to internet.

    let mut config = Ini::new();
    config.load("reticulum_dummy_config.conf")?;
    
    match c_type {
        peer::PeerCapability::Desktop => {
            config.set("reticulum", "enable_transport", Some("No".to_owned()));
            config.set("reticulum", "share_instance", Some("No".to_owned()));
        }

        peer::PeerCapability::Client | peer::PeerCapability::PtpRelay => {
            config.set("reticulum", "enable_transport", Some("Yes".to_owned()));
        }

        _ => {}
    }

    // TODO: when loglevel is actually implemented for prefengine, make the prefengine loglevel match reticulum's tiers
    config.set("logging", "loglevel", Some(log_level.to_string()));

    // TODO: reticulum only supports hardcoded auth passphrases on file, later on this needs to be env var and not just based 
    // off a file
    config.set("TCP Client Interface", "passphrase", Some(auth_pass));

    match ipv6_addr {
        Some(addr) => {
            config.set("TCP Client Interface", "listen_ip", Some(addr.to_string()));
            config.set("TCP Client Interface", "target_host", Some(addr.to_string()));    
        }
        _ => {}
    }

    match config.write("reticulum_config.conf") {
        Ok(()) => Ok(()),
        Err(err) => {
            Err(err.to_string())
        }
    }
}
