/*
    FILE: commands.rs
    AUTHOR(s): electra_rta

    DESCRIPTION: defines the execution model for how commands are going to be run inside
                threads.
*/

use std::
{
    collections::HashMap,
    future::Future,
    pin::Pin,
};

use chrono::NaiveDateTime;
use twitch_irc::
{
    login::StaticLoginCredentials,
    message::{PrivmsgMessage}, SecureTCPTransport, TwitchIRCClient,
};
use colored::*;

use crate::helpers::to_lowercase_cow;
use crate::db_ops::*;
use crate::models::*;



pub type Twitch_Client = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;

// CALLBACK TYPES (Blocking, Non-Blocking) [Function Pointers]
//pub type Callback = fn(u8, PrivmsgMessage) -> anyhow::Result<String>;
pub type Callback = Box<dyn Fn(Command) -> Pin<Box<dyn Future<Output = anyhow::Result<String>> + 'static + Send + Sync,>> + 'static + Send + Sync,>;


pub struct EventHandler
{
    pub bot_nick: String,
    pub command_map: HashMap<String, Callback>,
}

impl EventHandler
{
    // Thank you Azuchang!
    // Callback returns a dynamic future
    // Future yields a Result that can be unwrapped into a String
    pub fn add_command<Cb, F>(&mut self, name: String, function: Cb)
    where
    Cb: Fn(Command) -> F + 'static + Send + Sync,
    F: Future<Output = anyhow::Result<String>> + 'static + Send + Sync,
    {
        let cb = Box::new(move |cmd| Box::pin(function(cmd)) as _);
        self.command_map.insert(name, cb);
    }

    /*
        COMMAND METHODOLOGY
            - Each command can have multiple different ouputs
            - '!' before a string will attempt to execute the command (USE THIS IF YOU JUST WANT 1 ESC CHAR)
            - '?' before a string will attempt to help the user use the command
            - '#' special command
            - '^' UNDECIDED IDEAS: possible escape for grammar
            - '~' UNDECIDED

        name will be a Unique ID -> Value {Dataset, Help message}

        DESCRIPTION: This function handles only the processing of the initial message passed and the
                    function pointer execution and then sends the returned string back to the IRC channel

        FOR COMMAND IMPLEMENTATIONS SEE ~/cmds/
    */
    pub async fn execute_command(&self, command: Command, client: Twitch_Client) -> anyhow::Result<()>
    {
        const COLOR_FLAG: bool = true;
        //const TRACK_CC: bool = true;
        let mut is_pic: bool = false;
        let mut error_message: Option<String> = None;
        let name = command.name();
        if let Some(callback) = self.command_map.get(name)
        {
            handle_bac_user_in_db(&command.msg.sender.name, &command.msg.sender.id); // Updates user database
            // TODO: check if command is allowed in channel
            //let cmd_name = &name.clone();
            if let Some(cmd_id) = query_command_id(name)
            {
                //let channel_name = &command.msg.channel_login.clone()
                //let bacchannel: BACUser = query_user_data(command.msg.channel_login.clone());
                // IF WE FIND A CHANNEL COMMAND ENTRY HANDLE IT
                // IF WE DON'T FIND ONE INSERT IT THEN IGNORE AND CONTINUE
                if let Some(cc) = query_channel_command(&query_user_data(&command.msg.channel_login), cmd_id)
                {
                    let bacuser: BACUser = query_user_data(&command.msg.sender.name);
                    // COMMAND IS INACTIVE
                    if !cc.is_active && error_message.is_none()
                    {
                        error_message = Some(format!("This command is not available in {}\'s channel. Sorry {}", command.msg.channel_login, command.msg.sender.name));
                    }
                    let badges: Vec<&str> = command.msg.badges.iter().map(|b| b.name.as_str()).collect();
                    // COMMAND IS BROADCASTER ONLY
                    if cc.is_broadcaster_only && error_message.is_none() && !badges.contains(&"broadcaster")
                    {
                        error_message = Some(format!("This command is not available only to Broadcasters in {}\'s channel. Sorry {}", command.msg.channel_login, command.msg.sender.name));
                    }
                    // COMMAND IS BROADCASTER, MOD, VIP ONLY
                    if cc.is_mod_only && error_message.is_none() && !badges.contains(&"broadcaster") && !badges.contains(&"moderator") && !badges.contains(&"vip")
                    {
                        error_message = Some(format!("This command is not available to non-mods in {}\'s channel. Sorry {}", command.msg.channel_login, command.msg.sender.name));
                    }
                    if cmd_id == 7 // SKIP PIC COMMAND FOR NOW
                    {
                        is_pic = true;
                        //io_flag.0 = true;
                    }
                    // CHECK FOR TIMEOUT
                    if cc.has_timeout
                    {
                        let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
                        let timeout_out: (bool, i32) = handle_command_timeout(&query_user_data(&command.msg.channel_login), &bacuser, cmd_id, ndt_now, cc.timeout_dur);
                        if !timeout_out.0 // User has not waited for the timeout length
                        {
                            error_message = Some(format!("{}, please wait for {} more second(s)", command.msg.sender.name, cc.timeout_dur - timeout_out.1));
                        }
                    }
                }
                else // VALIDATED
                {
                    let bac_channel = query_user_data(&command.msg.channel_login);
                    insert_channel_command(&bac_channel, cmd_id);
                }
            }

            let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
            match error_message
            {
                Some(error_message) if !is_pic =>
                {
                    match COLOR_FLAG
                    {
                        true => println!("[{}] #{} <{}>: {}", dt_fmt.truecolor(138, 138, 138), command.msg.channel_login.truecolor(117, 97, 158), self.bot_nick.red(), error_message),
                        false => println!("[{}] #{} <{}>: {}", dt_fmt, command.msg.channel_login, self.bot_nick, error_message),
                    }
                    client.say(command.msg.channel_login, error_message).await?;
                }
                _ =>
                {
                    let res = callback(command.clone()).await.unwrap();
                    if res.is_empty(){return Ok(());} // if we have nothing to send skip the send
                    match COLOR_FLAG
                    {
                        true => println!("[{}] #{} <{}>: {}", dt_fmt.truecolor(138, 138, 138), command.msg.channel_login.truecolor(117, 97, 158), self.bot_nick.red(), res),
                        false => println!("[{}] #{} <{}>: {}", dt_fmt, command.msg.channel_login, self.bot_nick, res),
                    }
                    client.say(command.msg.channel_login, res).await?;
                }
            }

        }
        Ok(())
    }

    // OLD IMPLEMENTATION FOR BACKWARDS COMPAT
    // TODO: REMOVE
    pub async fn execute_command_old(&self, command: Command, client: Twitch_Client) -> anyhow::Result<()>
    {
        let command_name_lowercase = to_lowercase_cow(command.name());
        if let Some(callback) = self.command_map.get(command_name_lowercase.as_ref())
        {
            // TODO: check if command is allowed in channel
            handle_bac_user_in_db(&command.msg.sender.name, &command.msg.sender.id); // Updates user database
            let channel_login = command.msg.channel_login.clone();
            let res = callback(command.clone()).await?;
            if res.is_empty(){return Ok(());} // if we have nothing to send skip the send
            let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
            const COLOR_FLAG: bool = true;
            match COLOR_FLAG
            {
                true => println!("[{}] #{} <{}>: {}", dt_fmt.truecolor(138, 138, 138), channel_login.truecolor(117, 97, 158), self.bot_nick.red(), res),
                false => println!("[{}] #{} <{}>: {}", dt_fmt, channel_login, self.bot_nick, res),
            }
            client.say(command.msg.channel_login, res).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub runtype: Runtype,
    pub msg: PrivmsgMessage,
}

impl Command {
    pub fn try_from_msg(msg: PrivmsgMessage) -> Option<Self> {
        let runtype = Runtype::try_from_msg(&msg.message_text)?;
        return Some(Self {
            runtype,
            msg,
        });
    }

    fn cmd_and_args(&self) -> (&str, &str) {
        let Some(msg) = self.msg.message_text.get(1..) else {
            return ("", "");
        };
        return match msg.split_once(' ') {
            Some((cmd, args)) => (cmd, args),
            None => (msg, ""),
        };
    }

    pub fn name(&self) -> &str {
        return self.cmd_and_args().0;
    }

    pub fn args(&self) -> &str {
        return self.cmd_and_args().1;
    }
}

// AZUCHANG'S BOILERPLATE
#[derive(Debug, Clone, Copy)]
pub enum Runtype
{
    Command,
    Help,
    Hash,
    Tilde,
}

impl Runtype
{
    pub fn try_from_msg(msg_ctx: &str) -> Option<Runtype>
    {
        let rt = match msg_ctx.bytes().next()?
        {
            b'!' => Self::Command,
            b'?' => Self::Help,
            b'#' => Self::Hash,
            b'~' => Self::Tilde,
            _ => return None,
        };
        return Some(rt);
    }
}

///////////////////////////////////////////////////////////////////////////////
//                          COMMAND IMPLEMENTATIONS                          //
///////////////////////////////////////////////////////////////////////////////
pub async fn test_command(runtype: u8, _msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            return Ok(String::from("Test Command Block"));
        },
        b'?' =>
        {
            return Ok(String::from("Test Help Block"));
        },
        b'#' =>
        {
            return Ok(String::from("Test Hash Block"));
        },
        b'~' =>
        {
            return Ok(String::from("Test Tilde Block"));
        },
        _ => Ok(String::from("")),
    }
}