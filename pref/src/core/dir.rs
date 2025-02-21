use std::path;

pub fn get_root_file_path(file: &str) -> path::PathBuf
{
    let mut root_path = std::env::current_dir()
            .expect("Unable to read current working directory");

    root_path.push(file);
    return root_path;
}


fn get_peers_from_disk() -> Option<Vec<PeerInfo>> 
{
    let json_path = get_peer_file_path();

    if (path::Path::new(&json_path).exists()) {
        let peers_str = get_peer_file_str();

        if (peers_str.is_empty()) {
            return None;
        }

        let json_array: Vec<PeerInfo> =
            serde_json::from_str(&peers_str).expect("peers JSON was not well-formatted");
        
        let mut disk_peers = vec![];

        // vm - 192.132.123.233:3501
        for value in json_array
        {
            let sp = PeerInfo {
                id: value.id,
                addr: value.addr,
                public_key: PublicKey((ssb_id::SSB_NET_ID)),
            };

            disk_peers.push(sp);
        }
        

        return Some(disk_peers);
    }

    else {
        fs::File::create(json_path).expect("Unable to create json peers file");
        return None;
    }
}
