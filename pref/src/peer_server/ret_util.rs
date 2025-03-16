
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
            config.set("reticulum", "enable_transport", Some("No"));
            config.set("reticulum", "share_instance", Some("No"));
        }

        peer::PeerCapability::Server | peer::PeerCapability::PtpRelay => {
            config.set("reticulum", "enable_transport", Some("Yes"));
        }
    }

    // TODO: when loglevel is actually implemented for prefengine, make the prefengine loglevel match reticulum's tiers
    config.set("logging", "loglevel", Some(log_level));

    // TODO: reticulum only supports hardcoded auth passphrases on file, later on this needs to be dynamic and not just based 
    // off a file
    config.set("TCP Server Interface", "passphrase", Some(auth_pass));

    match ipv6_addr {
        Some(addr) => {
            config.set("TCP Server Interface", "listen_ip", Some(addr));
            config.set("TCP Client Interface", "target_host", Some(addr));    
        }
    }

    return config.write("reticulum_config.conf");
}
