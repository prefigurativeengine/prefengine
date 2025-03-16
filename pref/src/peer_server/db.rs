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

use crate::core::dir;
use crate::peer_server::Server;

enum FailedReason {
    IncorrectVisibility,
    MsgMalformed,
    MsgTooBig
}

pub fn process_local_change(changes: String, server: &mut Server) -> Result<(), String> {
    let mut fail_reason: Option<FailedReason> = None;
    validate_local_chg(&changes, fail_reason);

    if let Err(err) = send_chg_to_overlay(&changes, server) {
        Err(err)
    }
    
    let overlay_confirmation_res = listen_for_confirm_fin();
    if overlay_confirmation_res.is_ok() {
        append_chg(changes)
    }
    overlay_confirmation_res
}

fn send_chg_to_overlay(chg: &str, server: &mut Server) -> Result<(), String> {
    server.send_db_change(chg);
}

fn listen_for_confirm_fin() -> Result<(), String> {
    // wait for thread channel message, return result
    Ok(())
}

fn on_confirm_fin() -> Result<(), String> {
    // callback called from server listener thread
    Ok(())
}

// TODO: validations and confirms not implemented yet
pub fn process_remote_change(changes: String) -> Result<(), String> {
    // validate and checks
    
    let mut fail_reason: Option<FailedReason> = None;
    validate_remote_chg(&changes, fail_reason);


    // send success or err msg of above
    let overlay_confirmation_res: bool;
    if INITIATOR_CONFIRM {
        overlay_confirmation_res = initiator_confirm(&validated_map, fail_reason);
    } else {
        overlay_confirmation_res = peer_confirm(&validated_map, fail_reason);
    }

    // when all succeed msg recieved, push changes to disk
    if overlay_confirmation_res {
        write_remote_chg(validated_map);
    }
}

fn validate_remote_chg(msg: &str) {
    /*
    iniatior visibility
    size check (involving peer bandwidth check)
    lexical
    syntax
    semantics (involving matching procedure config check & consistent db log check)
    */

}

fn validate_local_chg(msg: &str) {
    /*
    self visibility
    size check
    lexical
    syntax
    semantics (involving peer bandwidth check)
    */

}

fn initiator_confirm(msg: &str, fail_reason: Option<FailedReason>) -> Result<(), String> {
    Ok(())
}

fn peer_confirm(msg: &str, fail_reason: Option<FailedReason>) -> Result<(), String> {
    Ok(())
}


fn write_remote_chg(msg: &HashMap<String, Value>) -> Result<(), String> {
    // append csv file
    let data_chg: String = msg.get("data").unwrap();
    return append_chg(data_chg);
}

pub fn append_chg(chg: String) -> Result<(), String>  {
    // TODO: maybe change this to a command sent to a db thread

    let db_path_res = dir::get_root_file_path("db.csv");
    match db_path_res {
        Ok(db_path) => {
            let db_csv_r = fs::read_to_string(db_path);
            if db_csv_r.is_err() {
                return db_csv_r;
            }

            let db_csv_s: &mut String = &mut db_csv_r.unwrap();

            let appended_db_csv = db_csv_s + chg;

            fs::write(db_path, appended_db_csv);
        }
        Err((msg)) => {
            log::error!(msg);
            Err(("Failed to get db from disk"));
        }
    }
}

pub fn db_to_str() -> Result<String, String> {
    let db_path_res = dir::get_root_file_path("db.csv");
    match db_path_res {
        Ok(db_path) => {
            let db_csv_r = fs::read_to_string(db_path);
            if db_csv_r.is_err() {
                db_csv_r;
            }

            return Ok(db_csv_r.unwrap())
        }
        Err((msg)) => {
            log::error!(msg);
            Err(("Failed to get db from disk"));
        }
    }
}

