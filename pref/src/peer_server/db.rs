use crate::core::dir;
use std::path::Path;
use std::fs::File;
use crate::peer_server::Client;
use serde_json::Error as s_Error;

// HACK: use SQL library in future instead of csv

const INITIATOR_CONFIRM: bool = true;
const DB_NAME: &'static str = "db.csv";


enum FailedReason {
    IncorrectVisibility,
    MsgMalformed,
    MsgTooBig
}

pub fn init() -> Result<(), String>  {
    // create file if not exists
    let db_path = Path::new(DB_NAME);
    let exist_res = Path::try_exists(db_path);
    if let Err(err) = exist_res {
        return Err(err.to_string())
    }

    let exist = exist_res.unwrap();
    if exist {
        Ok(())
    } else {
        let res = File::create(DB_NAME);
        if let Err(err) = res {
            return Err(err.to_string())
        }
        Ok(())
    }
}

pub fn process_local_change(changes: String, client: &mut Client) -> Result<(), String> {
    let mut fail_reason: Option<FailedReason> = None;
    validate_local_chg(&changes, fail_reason);

    if let Err(err) = send_chg_to_overlay(&changes, client) {
        return Err(err)
    }
    
    let overlay_confirmation_res = listen_for_confirm_fin();
    if overlay_confirmation_res.is_ok() {
        return append_chg(&changes)
    }
    overlay_confirmation_res
}

fn send_chg_to_overlay(chg: &str, client: &mut Client) -> Result<(), String> {
    return client.send_db_change(chg.to_owned());
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
    validate_remote_chg(&changes, &mut fail_reason);

    let change_map_r: Result<HashMap<String, Value>, s_Error> = serde_json::from_str(&changes);
    if let Err(err) = change_map_r {
        return Err(err.to_string());
    }

    let change_map: HashMap<String, Value> = change_map_r.unwrap();
    // send success or err msg of above
    if INITIATOR_CONFIRM {
        let confirm_res: Result<(), String> = initiator_confirm(fail_reason);

        // when all succeed msg recieved, push changes to disk
        match confirm_res {
            Ok(()) => {
                write_remote_chg(&change_map);
                Ok(())
            },
            Err(e) => {
                log::error!("Overlay confirm failed: {}", e);
                Err(e)
            }
        }
    } 
    else {
        let confirm_res: Result<(), String> = peer_confirm(fail_reason);
        match confirm_res {
            Ok(v) => {
                write_remote_chg(&change_map);
                Ok(())
            },
            Err(e) => {
                log::error!("Overlay confirm failed: {}", e);
                Err(e)
            }
        }
    }
}

fn validate_remote_chg(msg: &str, fail_reason: &mut Option<FailedReason>) {
    /*
    iniatior visibility
    size check (involving peer bandwidth check)
    lexical
    syntax
    semantics (involving matching procedure config check & consistent db log check)
    */

}

fn validate_local_chg(msg: &str, fail_reason: Option<FailedReason>) {
    /*
    self visibility
    size check
    lexical
    syntax
    semantics (involving peer bandwidth check)
    */

}

fn initiator_confirm(fail_reason: Option<FailedReason>) -> Result<(), String> {
    Ok(())
}

fn peer_confirm(fail_reason: Option<FailedReason>) -> Result<(), String> {
    Ok(())
}

use std::collections::HashMap;
use serde_json::Value;
fn write_remote_chg(msg: &HashMap<String, Value>) -> Result<(), String> {
    // append csv file
    if let Some(data) = msg.get("data") {
        if let Value::String(data_s) = data {
            return append_chg(data_s);
        } 
        
        else {
            return Err("incorrect value for id".to_owned()) 
        }
    }
    return Err("incorrect msg".to_owned()) 
}

pub fn append_chg(chg: &str) -> Result<(), String>  {
    // TODO: maybe change this to a command sent to a db thread

    let db_path_res = dir::get_root_file_path(DB_NAME);
    match db_path_res {
        Ok(db_path) => {
            let db_csv_r = std::fs::read_to_string(&db_path);
            if let Err(err) = db_csv_r {
                return Err(err.to_string());
            }

            let db_csv_s: String = db_csv_r.unwrap();

            let appended_db_csv = db_csv_s + chg;
            
            match std::fs::write(db_path, appended_db_csv) {
                Ok(()) => {
                    Ok(())
                },
                Err(e) => {
                    Err(e.to_string())
                }
            }

        }
        Err((msg)) => {
            return Err(("Failed to get db from disk".to_owned()));
        }
    }
}

pub fn db_to_str() -> Result<String, String> {
    let db_path_res = dir::get_root_file_path(DB_NAME);
    match db_path_res {
        Ok(db_path) => {
            let db_csv_r = std::fs::read_to_string(db_path);
            if db_csv_r.is_err() {
                return Err(("Failed to read csv".to_owned()));
            } else {
                return Ok(db_csv_r.unwrap())
            }
        }
        Err((msg)) => {
            return Err(("Failed to get db from disk".to_owned()));
        }
    }
}

