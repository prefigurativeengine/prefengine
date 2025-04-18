use std::{env::args, fmt};

#[derive(Debug)]
pub enum DiscoveryError {
    NetError(String),
    NATError(String),
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoveryError::NetError(error) => write!(f, "{}", error),

            DiscoveryError::NATError(error) => write!(f, "{}", error),
        }
    }
}

// Make it an error!
impl std::error::Error for DiscoveryError {}

#[derive(Debug)]
pub struct NATError {}

impl fmt::Display for NATError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NATError")
    }
}

impl Error for NATError {}

#[derive(Debug)]
pub struct NetError {}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NetError")
    }
}

impl Error for NetError {}

use crate::core::PREF_PEER_PORT;
use easy_upnp::{Ipv4Cidr, PortMappingProtocol, UpnpConfig, add_ports, delete_ports};
use log::error;
use std::error::Error;

// TODO: Try other methods of NAT traversal other than upnp, like nat-pmp, configuring ipv6, STUN, TURN

pub struct NATConfig {
    pub is_symmetric: bool,
    pub auto_port_forward: bool,
}

impl NATConfig {
    pub fn new() -> NATConfig {
        // TODO: implement, a website exists to get this
        let is_sym = false;

        match try_upnp_setup() {
            Ok(()) => Self {
                is_symmetric: is_sym,
                auto_port_forward: true,
            },
            Err(err) => Self {
                is_symmetric: is_sym,
                auto_port_forward: false,
            },
        }
    }
}

fn try_upnp_setup() -> Result<(), DiscoveryError> {
    let config: UpnpConfig = get_config();

    // easy_upnp can only add lists of ports
    let mut result = add_ports([config]);
    let first_port_res: Option<Result<(), easy_upnp::Error>> = result.next();

    if let Some(Err(err)) = first_port_res {
        error!("{}", err);
        return Err(DiscoveryError::NATError(err.to_string()));
    } else if let None = first_port_res {
        error!("add_ports returned None");
        return Err(DiscoveryError::NATError(
            "add_ports returned None".to_owned(),
        ));
    }

    Ok(())
}

use std::process::Command;
pub fn get_public_ip() -> Result<String, Box<dyn Error>> {
    let ip_vec = Command::new("curl").arg("ipinfo.io/ip").output()?.stdout;

    Ok(String::from_utf8(ip_vec)?)
}

fn get_config() -> UpnpConfig {
    // TODO: upnp setup takes very long; try to use specific address
    let config_no_address = UpnpConfig {
        address: None,
        port: PREF_PEER_PORT,
        protocol: PortMappingProtocol::TCP,
        duration: 3600,
        comment: "Client".to_string(),
    };

    config_no_address
}

pub fn rmv_upnp_setup() -> Result<(), DiscoveryError> {
    let config: UpnpConfig = get_config();
    let mut result = delete_ports([config]);

    let first_port_res: Option<Result<(), easy_upnp::Error>> = result.next();

    // TODO: shouldn't happen but check if None
    let unwrap_res = first_port_res.unwrap();

    match unwrap_res {
        Ok(()) => Ok(()),
        Err(err) => {
            return Err(DiscoveryError::NATError(err.to_string()));
        }
    }
}
