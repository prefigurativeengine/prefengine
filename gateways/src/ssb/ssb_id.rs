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



pub const GATE_NET_ID: [usize; 32] = [
    0x53,
    0x4d,
    0x61,
    0x72,
    0x41,
    0x79,
    0x70,
    0x4c,
    0x65,
    0x64,
    0x59,
    0x55,
    0x79,
    0x4f,
    0x78,
    0x6a,
    0x69,
    0x74,
    0x79,
    0x2b,
    0x68,
    0x74,
    0x72,
    0x7a,
    0x51,
    0x62,
    0x6f,
    0x5a,
    0x47,
    0x79,
    0x30,
    0x2f
];

pub const SSB_NET_ID: [usize; 32] = [
    0xd4,
    0xa1,
    0xcb,
    0x88,
    0xa6,
    0x6f,
    0x02,
    0xf8,
    0xdb,
    0x63,
    0x5c,
    0xe2,
    0x64,
    0x41,
    0xcc,
    0x5d,
    0xac,
    0x1b,
    0x08,
    0x42,
    0x0c,
    0xea,
    0xac,
    0x23,
    0x08,
    0x39,
    0xb7,
    0x55,
    0x84,
    0x5a,
    0x9f,
    0xfb
];