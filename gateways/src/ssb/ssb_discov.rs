use kuska_ssb::discovery::Invite;

// only invitecode will be implemented for now
pub fn parse_invitecode(invitecode: &str) 
{
    let invite: Invite = Invite::from_code(invitecode);
    return invite;
}

// make invitecode here 
// 1. get ip addr (TODO: maybe add domain support later)
// 2. get ssb port
// 3. get key pair 

pub enum SSBDiscoveryMethod
{
    LANBroadcast,
    InviteCode,
    BluetoothBroadcast
}
