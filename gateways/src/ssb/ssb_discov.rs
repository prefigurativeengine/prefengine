use kuska_ssb::discovery::Invite;


fn parse_invitecode(invitecode: &str) 
{
    let invite: Invite = Invite::from_code(invitecode);
    return invite;
}
