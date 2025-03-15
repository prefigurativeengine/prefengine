/*

have single json file for protype, which has various entities, which contains arbitrary number of 
keys and values. there will be crud methods for entity values and kv's. accordant net msgs will be 
done at end of method with json str

the ddb system as a whole will intake data from the sandbox, and do two things with the userinput; 
submitting the data changes to the local cache/stash (maybe able to be temporarily on disk), then 
once checks are done there, the data changes are submitted to the network. once confirmation is 
finished from that server, changes are saved to actual local database.

this process happens in reverse (starting from server) for recieving data changes, except sandbox is 
not touched, for now

*/

const INITIATOR_CONFIRM: bool = true;

// TODO: validations and confirms not implemented yet
pub fn process_remote_change(changes: String) -> Result<(), String> {
    // validate and checks
    
    let validated_map_r = match netspace_validate(&changes).map_err(op)
    if validated_map_r.is_err() {
        validated_map_r
    }
    let validated_map = validated_map_r.unwrap();

    if let Err(err) = localspace_validate(&validated_map).is_err() {
        Err((err))
    }

    // send success or err msg of above
    let overlay_confirmation_res: bool;
    if INITIATOR_CONFIRM {
        overlay_confirmation_res = initiator_confirm(&validated_map);
    } else {
        overlay_confirmation_res = peer_confirm(&validated_map);
    }

    // when all succeed msg recieved, push changes to disk
    if overlay_confirmation_res {
        push_to_db(validated_map);
    }
}

fn netspace_validate(msg: &str) -> Result<HashMap<String, Value>, String> {
    /*
    iniatior visibility
    size check (involving peer bandwidth check)
    lexical
    syntax
    semantics (involving matching procedure config check)
    */

    let serde_res: Result<HashMap<String, Value>, Error> = serde_json::from_str(msg);
    match serde_res {
        Ok(serde_s) => {
            Ok((serde_s))
        }
        Err((msg)) => {
            log::error!(msg);
            Err(("Failed to parse db change string"));
        }
    }
    
}

fn localspace_validate(msg: &str) -> Result<(), String> {
    /*
    semantics (involving consistent db log check)
    */
    Ok(())
}

fn initiator_confirm(msg: &str) -> Result<(), String> {
    Ok(())
}

fn peer_confirm(msg: &str) -> Result<(), String> {
    Ok(())
}


fn push_to_db(msg: &HashMap<String, Value>) {
    // append csv file
    let db_path_res = dir::get_root_file_path("db.csv");
    match db_path_res {
        Ok(db_path) => {
            let db_csv_r = fs::read_to_string(db_path);
            if db_csv_r.is_err() {
                return db_csv_r;
            }

            let db_csv_s: &mut String = &mut db_csv_r.unwrap();
            let data_chg: String = msg.get("data").unwrap();

            let appended_db_csv = db_csv_s + data_chg;

            fs::write(db_path, appended_db_csv);
        }
        Err((msg)) => {
            log::error!(msg);
            Err(("Failed to get db from disk"));
        }
    }
}


pub fn process_local_change() {
    // validate and checks

    // if success, store in temp cache

    // send broadcast to relevant peers

    // wait for relevant peer response

    // when all msg recieved, send total peer process results out to relevants

    // if all succeed, push changes to disk
}

