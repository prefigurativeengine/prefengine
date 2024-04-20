use std::fs::File;
use std::io::BufWriter;
use std::{error::Error, path::PathBuf};
use std::path::Path;

use kuska_handshake;
use kuska_handshake::sodiumoxide::crypto::box_::{PublicKey, SecretKey};
use kuska_ssb::keystore::{self, OwnedIdentity};
use sodiumoxide::crypto::{sign::ed25519};
use dirs_next;
use tokio::io::BufReader;

// TODO: make error enum for this
// TODO: move this to generic file src dir
fn get_home_dir() -> Result<PathBuf, String>
{
    #[cfg(target_os = "windows")]
    let slash = "\\";

    #[cfg(not(target_os = "windows"))]
    let slash = "/";

    let home_dir: PathBuf = dirs_next::home_dir().expect("User home directory should be readable");
    return Ok(home_dir);
}

fn get_ssb_secret_path() -> Result<PathBuf, ()>
{
    if let Ok(mut home_dir) = get_home_dir() {
        home_dir.push(".ssb");
        home_dir.push("secret");
        return Ok(home_dir);
    }
    return Err(())
}

pub fn get_ssb_id() -> Result<OwnedIdentity, String>
{
    let result = get_ssb_secret_path();
    if result == Err(()) {
        return Err("Failed to read ssbsecret directory.".to_owned());
    }

    let mut ssb_secret_p: PathBuf = result.unwrap();

    let ssb_secret_f = File::create(ssb_secret_p).expect("Unable to create ssb secret file");

    use tokio::io::BufReader;
    let mut ssb_reader: BufReader<File> = BufReader::new(ssb_secret_f);

    let id: OwnedIdentity = kuska_ssb::keystore::read_patchwork_config(&mut ssb_reader).await;

    return Ok(id);
}


pub fn first_time_id_gen() -> Result<(), String>
{
    let kp_struct: OwnedIdentity = kuska_ssb::keystore::OwnedIdentity::create();

    let result = get_ssb_secret_path();
    if result == Err(()) {
        return Err("Failed to read ssbsecret directory.".to_owned());
    }

    let mut ssb_secret_p: PathBuf = result.unwrap();

    let ssb_secret_f = File::create(ssb_secret_p).expect("Unable to create ssb secret file");
    let mut ssb_secret_w: BufWriter<File> = BufWriter::new(ssb_secret_f);

    kuska_ssb::keystore::write_patchwork_config(&kp_struct, &mut ssb_secret_w);
    return Ok(())
}
