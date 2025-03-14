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

const text_change_size_limit: u32 = 50000;

pub fn process_remote_change(changes: String) -> Result<(), String> {
    // validate and checks
    
    if let Err(err) = netspace_validate(&changes).is_err() {
        Err((err))
    }

    if let Err(err) = localspace_validate(&changes).is_err() {
        Err((err))
    }

    // send success or err msg of above to initiator
    let overlay_confirmation_res: bool;
    if initiator_confirm {
        overlay_confirmation_res = initiator_confirm(&changes);
    } else {
        overlay_confirmation_res = peer_confirm(&changes);
    }

    // when all succeed msg recieved, push changes to disk
    if overlay_confirmation_res {
        push_to_sql();
    }
}

fn netspace_validate(msg: String) -> Result<(), String> {
    /*
    iniatior visibility
    size check (involving peer bandwidth check)
    &HashMap<
    */

    if 
    /* 
    lexical
    syntax
    semantics (involving matching procedure config check)
    */
}

fn localspace_validate(msg: &HashMap<String, Value>) -> Result<(), String> {
    /*
    semantics (involving consistent changelog check)
    */
}






pub fn process_local_change() {
    // validate and checks

    // if success, store in temp cache

    // send broadcast to relevant peers

    // wait for relevant peer response

    // when all msg recieved, send total peer process results out to relevants

    // if all succeed, push changes to disk
}

