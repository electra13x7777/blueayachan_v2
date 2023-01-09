/*
    FILE: commands.rs
    AUTHOR(s): electra_rta

    DESCRIPTION: defines the execution model for how commands are going to be run inside
                threads.
*/

use std::
{
    collections::HashMap,
    io::{prelude::*},
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


use crate::db_ops::*;
use crate::models::*;



pub type Twitch_Client = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;

// CALLBACK TYPES (Blocking, Non-Blocking) [Function Pointers]
//pub type Callback = fn(u8, PrivmsgMessage) -> anyhow::Result<String>;
pub type Callback = Box<dyn Fn(u8, PrivmsgMessage) -> Pin<Box<dyn Future<Output = anyhow::Result<String>> + 'static + Send + Sync,>> + 'static + Send + Sync,>;


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
    Cb: Fn(u8, PrivmsgMessage) -> F + 'static + Send + Sync,
    F: Future<Output = anyhow::Result<String>> + 'static + Send + Sync,
    {
        let cb = Box::new(move |a, b| Box::pin(function(a, b)) as _);
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
    pub async fn execute_command(&self, name: String, client: Twitch_Client, msg_ctx: PrivmsgMessage) -> anyhow::Result<()>
    {
        const COLOR_FLAG: bool = true;
        //const TRACK_CC: bool = true;
        let mut io_flag: (bool, String) = (false, "".to_string());
        if self.command_map.contains_key(&name)
        {
            handle_bac_user_in_db(msg_ctx.sender.name.clone(), msg_ctx.sender.id.clone()); // Updates user database
            // TODO: check if command is allowed in channel
            //let cmd_name = &name.clone();
            let cmd_id: i32 = query_command_id(&name).unwrap_or(0);
            if cmd_id != 0 && !io_flag.0
            {
                //let channel_name = &msg_ctx.channel_login.clone()
                //let bacchannel: BACUser = query_user_data(msg_ctx.channel_login.clone());
                // IF WE FIND A CHANNEL COMMAND ENTRY HANDLE IT
                // IF WE DON'T FIND ONE INSERT IT THEN IGNORE AND CONTINUE
                if let Some(cc) = query_channel_command(query_user_data(msg_ctx.channel_login.clone()), cmd_id)
                {
                    let bacuser: BACUser = query_user_data(msg_ctx.sender.name.clone());
                    // COMMAND IS INACTIVE
                    if !cc.is_active && !io_flag.0
                    {
                        io_flag.0 = !io_flag.0;
                        io_flag.1 = format!("This command is not available in {}\'s channel. Sorry {}", msg_ctx.channel_login, msg_ctx.sender.name);
                    }
                    let badges: Vec<String> = msg_ctx.badges.iter().map(|b| b.name.clone()).collect();
                    // COMMAND IS BROADCASTER ONLY
                    if cc.is_broadcaster_only && !io_flag.0 && !badges.contains(&"broadcaster".to_string()) {
                        io_flag.0 = !io_flag.0;
                        io_flag.1 = format!("This command is not available only to Broadcasters in {}\'s channel. Sorry {}", msg_ctx.channel_login, msg_ctx.sender.name);
                    }
                    // COMMAND IS BROADCASTER, MOD, VIP ONLY
                    if cc.is_mod_only && !io_flag.0 && !badges.contains(&"broadcaster".to_string()) && !badges.contains(&"moderator".to_string()) && !badges.contains(&"vip".to_string()) {
                        io_flag.0 = !io_flag.0;
                        io_flag.1 = format!("This command is not available to non-mods in {}\'s channel. Sorry {}", msg_ctx.channel_login, msg_ctx.sender.name);
                    }
                    if cmd_id == 7 // SKIP PIC COMMAND FOR NOW
                    {
                        io_flag.0 = true;
                    }
                    // CHECK FOR TIMEOUT
                    if cc.has_timeout
                    {
                        let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
                        let timeout_out: (bool, i32) = handle_command_timeout(query_user_data(msg_ctx.channel_login.clone()), bacuser, cmd_id, ndt_now, cc.timeout_dur);
                        if !timeout_out.0 // User has not waited for the timeout length
                        {
                            io_flag.0 = !io_flag.0;
                            io_flag.1 = format!("{}, please wait for {} more second(s)", msg_ctx.sender.name, cc.timeout_dur - timeout_out.1);
                        }
                    }
                }
                else // VALIDATED
                {
                    let bac_channel = query_user_data(msg_ctx.channel_login.clone());
                    insert_channel_command(bac_channel, cmd_id);
                }
            }

            let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
            if !io_flag.0
            {
                const COMMAND_INDEX: usize = 0;
                let runtype: u8 = msg_ctx.message_text.clone().as_bytes()[COMMAND_INDEX]; // gets a byte literal (Ex. b'!')
                let callback = self.command_map.get(&name).expect("Could not execute function pointer!");
                let res = callback(runtype, msg_ctx.clone()).await.unwrap();
                if res.is_empty(){return Ok(());} // if we have nothing to send skip the send
                match COLOR_FLAG
                {
                    true => println!("[{}] #{} <{}>: {}", dt_fmt.truecolor(138, 138, 138), msg_ctx.channel_login.truecolor(117, 97, 158), self.bot_nick.red(), res),
                    false => println!("[{}] #{} <{}>: {}", dt_fmt, msg_ctx.channel_login, self.bot_nick, res),
                }
                client.say(msg_ctx.channel_login.clone(), res.to_string()).await?;
            }
            else
            {
                match COLOR_FLAG
                {
                    true => println!("[{}] #{} <{}>: {}", dt_fmt.truecolor(138, 138, 138), msg_ctx.channel_login.truecolor(117, 97, 158), self.bot_nick.red(), io_flag.1),
                    false => println!("[{}] #{} <{}>: {}", dt_fmt, msg_ctx.channel_login, self.bot_nick, io_flag.1),
                }
                client.say(msg_ctx.channel_login.clone(), io_flag.1.to_string()).await?;
            }

        }
        Ok(())
    }

    // OLD IMPLEMENTATION FOR BACKWARDS COMPAT
    // TODO: REMOVE
    pub async fn execute_command_old(&self, name: String, client: Twitch_Client, msg: PrivmsgMessage) -> anyhow::Result<()>
    {
        if self.command_map.contains_key(&name)
        {
            // TODO: check if command is allowed in channel
            handle_bac_user_in_db(msg.sender.name.clone(), msg.sender.id.clone()); // Updates user database
            const COMMAND_INDEX: usize = 0;
            let runtype: u8 = msg.message_text.clone().as_bytes()[COMMAND_INDEX]; // gets a byte literal (Ex. b'!')
            let callback = self.command_map.get(&name).expect("Could not execute function pointer!");
            let res = callback(runtype, msg.clone()).await.unwrap();
            if res.is_empty(){return Ok(());} // if we have nothing to send skip the send
            let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
            const COLOR_FLAG: bool = true;
            match COLOR_FLAG
            {
                true => println!("[{}] #{} <{}>: {}", dt_fmt.truecolor(138, 138, 138), msg.channel_login.truecolor(117, 97, 158), self.bot_nick.red(), res),
                false => println!("[{}] #{} <{}>: {}", dt_fmt, msg.channel_login, self.bot_nick, res),
            }
            client.say(msg.channel_login.clone(), res.to_string()).await?;
        }
        Ok(())
    }
}

// AZUCHANG'S BOILERPLATE
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