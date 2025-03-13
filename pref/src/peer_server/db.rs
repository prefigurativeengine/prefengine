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



pub fn process_remote_change(changes: HashMap<String, Value>) {
    // validate and checks

    // send success or err msg of above to initiator

    // when all succeed msg recieved, push changes to disk
}


pub fn process_local_change() {
    // validate and checks

    // if success, store in temp cache

    // send broadcast to relevant peers

    // wait for relevant peer response

    // when all msg recieved, send total peer process results out to relevants

    // if all succeed, push changes to disk
}

