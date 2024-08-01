use twitch_irc::message::PrivmsgMessage;

// BLUEAYACHAN CHANNEL ONLY COMMANDS
pub async fn add_bot(_runtype: u8, _msg_ctx: PrivmsgMessage) -> anyhow::Result<String> {
    return Ok(String::from(""));
}
pub async fn remove_bot(_runtype: u8, _msg_ctx: PrivmsgMessage) -> anyhow::Result<String> {
    return Ok(String::from(""));
}
