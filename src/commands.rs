/*
    FILE: commands.rs
    AUTHOR(s): electra_rta

    DESCRIPTION: defines the execution model for how commands are going to be run inside
                threads.
*/
use anyhow::Context;
use std::
{
    fs,
    fs::File,
    path::Path,
    env,
    time::Duration,
    collections::HashMap,
    io,
    io::{prelude::*, BufReader, Write},
    future::Future,
    pin::Pin,
};
use rand::Rng;
use chrono::NaiveDateTime;
use twitch_irc::
{
    login::StaticLoginCredentials,
    message::{PrivmsgMessage, ServerMessage},
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};
use colored::*;

use crate::helpers::readlines_to_vec;
use crate::db_ops::*;
use crate::models::*;

use crate::cmds::cmd_externalquery::*;

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
        if let Some(callback) = self.command_map.get(&command.name_lowercase)
        {
            // TODO: check if command is allowed in channel
            handle_bac_user_in_db(&command.sender_name_lowercase, &command.msg.sender.id); // Updates user database
            const COMMAND_INDEX: usize = 0;
            let channel_login = command.msg.channel_login.clone();
            let res = callback(command).await?;
            if res.is_empty() {return Ok(());} // if we have nothing to send skip the send
            let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
            const COLOR_FLAG: bool = true;
            match COLOR_FLAG
            {
                true => println!("[{}] #{} <{}>: {}", dt_fmt.truecolor(138, 138, 138), channel_login.truecolor(117, 97, 158), self.bot_nick.red(), res),
                false => println!("[{}] #{} <{}>: {}", dt_fmt, channel_login, self.bot_nick, res),
            }
            client.say(channel_login, res).await?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Command {
    pub name_lowercase: String,
    pub args: String,
    pub runtype: Runtype,
    pub sender_name_lowercase: String,
    pub msg: PrivmsgMessage,
}

impl Command {
    pub fn try_from_msg(msg: PrivmsgMessage) -> Option<Self> {
        let runtype = Runtype::try_from_msg(&msg.message_text)?;
        let contents = msg.message_text.get(1..)?;
        let (name, args) = match contents.split_once(' ') {
            Some((name, args)) => (name, args),
            None => (contents, ""),
        };
        Some(Self {
            name_lowercase: name.to_lowercase(),
            args: args.to_string(),
            runtype,
            sender_name_lowercase: msg.sender.name.to_lowercase(),
            msg,
        })
    }
}

// AZUCHANG'S BOILERPLATE
#[derive(Debug)]
pub enum Runtype
{
    Command,
    Help,
    Hash,
    Tilde,
}

impl Runtype
{
    pub fn try_from_msg(msg: &str) -> Option<Runtype>
    {
        let rt = match msg.bytes().next()?
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
pub async fn test_command(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
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