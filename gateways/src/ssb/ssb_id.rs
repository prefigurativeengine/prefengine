use std::{error::Error, path::PathBuf};
use std::path::Path;

use kuska_handshake;
use kuska_handshake::sodiumoxide::crypto::box_::{PublicKey, SecretKey};
use kuska_ssb::keystore;
use sodiumoxide::crypto::{sign::ed25519};
use dirs_next;

fn get_home_dir() -> Result<String, String>
{
    #[cfg(target_os = "windows")]
    let slash = "\\";

    #[cfg(not(target_os = "windows"))]
    let slash = "/";

    let home_dir: PathBuf = dirs_next::home_dir().expect("User home directory should be writeable");
    return Ok(home_dir);
}

fn get_ind_ssb_path() -> PathBuf
{
    if let Some(home_dir) = get_home_dir() {
        return home_dir.push(".ssb");
    }

}

fn write_path_if_not_exist(path: &str) -> bool
{
    let exists: bool = Path::new(path).exists();
    if (exists) {
        return true;
    } else {
        std::fs::create_dir(path)
            .expect("Should have been able to write the path");
        return false;
    }
}

fn write_to_ssbsecret(key_pair: u32) -> std::io::Result<()>
{
    use std::fs;
    let mut ssb_path: PathBuf = get_ind_ssb_path();
    // if not there, create path and write file
    // if there, ask user if original or new one to be created, and inform of risk
    let exists: bool = write_path_if_not_exist(&ssb_path);
    
    if (!exists)
    {
        ssb_path.push("secret");
        std::fs::write(ssb_path, &key_pair.to_string())
            .expect("Should have been able to read the file");
        return Ok(());
    }
    else 
    {
        return Err(());
    }
}


pub fn first_time_id_gen()
{
    let kp_struct: OwnedIdentity = kuska_ssb::keystore::OwnedIdentity::create();

    // Write and Unpin trait for writer
    kuska_ssb::keystore::write_patchwork_config(kp_struct, writer)
}
