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
};
use rand::Rng;
//use chrono;
use twitch_irc::
{
    login::StaticLoginCredentials,
    message::{PrivmsgMessage, ServerMessage},
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};
use crate::helpers::readlines_to_vec;
pub type Twitch_Client = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;
// uses an unsigned 8bit int to signify what block to execute
pub type Callback = fn(u8) -> String;// Option<String>;

pub struct EventHandler
{
    pub command_map: HashMap<String, Callback>,
}

impl EventHandler
{
    pub fn add_command(&mut self, name: String, function: Callback)
    {
        self.command_map.insert(name, function);
    }
}


/*
    COMMAND METHODOLOGY
        - Each command can have multiple different ouputs
        - '!' before a string will attempt to execute the command (USE THIS IF YOU JUST WANT 1 ESC CHAR)
        - '?' before a string will attempt to help the user use the command
        - '^' UNDECIDED IDEAS: possible escape for grammar
        - '~' UNDECIDED

    name will be a Unique ID -> Value {Dataset, Help message}

    DESCRIPTION: This function handles only the processing of the initial message passed and the
                 function pointer execution and then sends the returned string back to the IRC channel
*/
pub async fn execute_command(name: String, client: Twitch_Client, msg: PrivmsgMessage, cmd_map: HashMap<String, Callback>) -> anyhow::Result<()>
{
    let command_index: usize = 0;
    let runtype: u8 = msg.message_text.as_bytes()[command_index]; // gets a byte literal (Ex. b'!')
    if cmd_map.contains_key(&name as &str)
    {
        let out = cmd_map.get(&name as &str).expect("Some shit went wrong!");
        let res = String::from(out(runtype));
        let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
        println!("[{}] #{} <blueayachan>: {}", dt_fmt, msg.channel_login, res);
        client.say(
                msg.channel_login.clone(),
        format!("{}", res)
        ).await?;
    }
    Ok(())
}

pub fn test_command(runtype: u8) -> String
{
    match runtype
    {
        b'!' =>
        {
            return String::from("Test Command Block")
        },
        b'?' =>
        {
            return String::from("Test Help Block")
        },
        _ => {return String::from("");},
    }
}

// Preliminary implementation of dreamboumtweet (will eventually change)
// TODO: possibly add parameter usage: u8
pub fn dreamboumtweet(runtype: u8) -> String//Option<String>//(String, String)
{
    match runtype
    {
        b'!' =>
        {
            // read in list of tweets
            let mut dbt_vec = readlines_to_vec("dreamboum_tweets_10_05_2022.txt").expect("Could not load lines");
            let index = rand::thread_rng().gen_range(0..dbt_vec.len());
            let splitpoint: usize = 13;
            let length = dbt_vec[index].len();
            let tweet_ctx: &str = &dbt_vec[index][0..length-splitpoint];
            //let date_str: &str = &dbt_vec[index][length-splitpoint..];
            //return Some(tweet_ctx.to_string());
            return tweet_ctx.to_string();
        },
        b'?' =>
        {
            //return Some("This command randomly sends a tweet made by twitter user @Dreamboum".to_string());
            return String::from("This command sends a random tweet made by twitter user @Dreamboum");
        },
        _ =>
        {
            return String::from("");
        },
    }
}