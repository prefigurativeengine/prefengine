use kuska_handshake;
use sodiumoxide::crypto::{sign::ed25519};

fn write_to_ssbsecret(key_pair: u32) -> std::io::Result<()>
{
    use std::fs;
    let ssbsecret_path: String = get_ind_ssbsecret_path();
    // if not there, create path and write file
    // if there, ask user if original or new one to be created, and inform of risk
    let exists: bool = write_path_if_not_exist(&ssbsecret_path);
    
    if (!exists)
    {
        std::fs::write(ssbsecret_path, &key_pair.to_string())
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
    let (pk, sk) = ed25519::gen_keypair();
}