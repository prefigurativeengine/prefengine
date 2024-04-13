use std::fs::File;
use std::io::BufWriter;
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

    let home_dir: PathBuf = dirs_next::home_dir().expect("User home directory should be readable");
    return Ok(home_dir);
}

fn get_ssb_home_path() -> PathBuf
{
    if let Some(home_dir) = get_home_dir() {
        return home_dir.push(".ssb");
    }

}


pub fn first_time_id_gen()
{
    let kp_struct: OwnedIdentity = kuska_ssb::keystore::OwnedIdentity::create();

    let ssb_secret_p: PathBuf = get_ssb_home_path();
    ssb_secret_p.push("secret");

    let ssb_secret_f = File::create(ssb_secret_p).expect("Unable to create ssb secret file");
    let ssb_secret_w: BufWriter<File> = BufWriter::new(ssb_secret_f);

    kuska_ssb::keystore::write_patchwork_config(&kp_struct, ssb_secret_w);
}
