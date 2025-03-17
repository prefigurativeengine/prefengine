use std::fmt as fmt;

#[derive(Debug)]
pub enum DiscoveryError {
    NetError(String),
    NATError(String),
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoveryError::NetError(error) => 
                write!(f, "{}", error),

            DiscoveryError::NATError(error) => 
                write!(f, "{}", error),
        }
    }
}

// Make it an error!
impl std::error::Error for DiscoveryError {}

#[derive(Debug)]
pub struct NATError {  }

impl fmt::Display for NATError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NATError")
    }
}

impl Error for NATError {}

#[derive(Debug)]
pub struct NetError { }

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NetError")   
    }
}

impl Error for NetError {}

pub struct DiscoveryResult 
{
    pub upnp_enabled: bool
}

use std::error::Error;
use log::error;
use easy_upnp::{add_ports, delete_ports, PortMappingProtocol, UpnpConfig};
use crate::core::PREF_PEER_PORT;

fn get_config() -> UpnpConfig {
    let config_no_address = UpnpConfig {
        address: None,
        port: PREF_PEER_PORT,
        protocol: PortMappingProtocol::TCP,
        duration: 3600,
        comment: "Server".to_string(),
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
            return Err(
                DiscoveryError::NATError(err.to_string())
            );
        }
    }
}

pub fn try_upnp_setup() -> Result<String, DiscoveryError> {
    let config: UpnpConfig = get_config();

    // easy_upnp can only add lists of ports
    let mut result = add_ports([config]);
    let first_port_res: Option<Result<(), easy_upnp::Error>> = result.next();

    if let Some(Err(err)) = first_port_res {
        error!("{}", err);
        return Err(
            DiscoveryError::NATError(err.to_string())
        );
    } 
    else if let None = first_port_res {
        error!("add_ports returned None");
        return Err(
            DiscoveryError::NATError("add_ports returned None".to_owned())
        );
    }

    let res: Result<String, Box<dyn Error>> = get_public_ip();

    match res {
        Ok((res)) => Ok(res),
        Err(err) => {
            error!("{}", err);
            return Err(
                DiscoveryError::NetError(err.to_string())
            );
        }
    }
}


fn get_public_ip() -> Result<String, Box<dyn Error>> {
    let res = reqwest::blocking::get("http://myexternalip.com/raw")?.text()?;

    Ok(res)
}

