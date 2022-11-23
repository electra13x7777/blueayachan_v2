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
pub type Callback = fn(String) -> String;// Option<String>;

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
        - Each command has 2 different ouputs
        - '!' before a string will attempt to execute the command
        - '?' before a string will attempt to help the user use the command
        - '^' UNDECIDED IDEAS: possible escape for grammar
        - '~' UNDECIDED

    name will be a Unique ID -> Value {Dataset, Help message}
*/
pub async fn execute_command(name: String, client: Twitch_Client, msg: PrivmsgMessage, cmd_map: HashMap<String, Callback/*(String, String)Box<Command>*/>) -> anyhow::Result<()>
{
    let command_index: usize = 0;
    let runtype_u8: u8 = msg.message_text.as_bytes()[command_index];
    //let runtype: char = runtype_u8 as char;
    //if let Some(out) = cmd_map.get(&name as &str).expect("Some shit went wrong!")
    if cmd_map.contains_key(&name as &str)
    {
        let out = cmd_map.get(&name as &str).expect("Some shit went wrong!");
        match runtype_u8
        {
            // Command Block
            b'!' =>
            {
                let res = String::from(out(runtype_u8.to_string()));
                let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
                println!("[{}] #{} <blueayachan>: {}", dt_fmt, msg.channel_login, res);
                client.say(
                        msg.channel_login.clone(),
                format!("{}", res)
                ).await?;
            },
            // Help Block
            b'?' =>
            {
                let res = String::from(out(runtype_u8.to_string()));
                let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
                println!("[{}] #{} <blueayachan>: {}", dt_fmt, msg.channel_login, res);
                client.say(
                    msg.channel_login.clone(),
                    format!("{}", res),
                ).await?;
            },
            _ =>
            {
                // should never execute
            },
        }
    }
    Ok(())
}

// Preliminary implementation of dreamboumtweet (will eventually change)
// TODO: possibly add parameter usage: u8
pub fn dreamboumtweet(runtype_str: String) -> String//Option<String>//(String, String)
{
    let runtype: u8 = runtype_str.as_bytes()[0];
    //let runtype: &str = runtype_u8 as str;
    //println!("{}", runtype);
    match runtype
    {
        // !
        51 =>
        {
            // read in list of tweets
            let mut dbt_vec = readlines_to_vec("dreamboum_tweets_10_05_2022.txt").expect("Could not load lines");
            let index = rand::thread_rng().gen_range(0..dbt_vec.len());
            // create temporary file to write html to
            let splitpoint: usize = 13;
            let length = dbt_vec[index].len();
            let tweet_ctx: &str = &dbt_vec[index][0..length-splitpoint];
            //let date_str: &str = &dbt_vec[index][length-splitpoint..];
            //return Some(tweet_ctx.to_string());
            return tweet_ctx.to_string()
        },
        // ?
        54 =>
        {
            //return Some("This command randomly sends a tweet made by twitter user @Dreamboum".to_string());
            return "This command randomly sends a tweet made by twitter user @Dreamboum".to_string();
        },
        _ =>
        {
            "".to_string()//None
        },
    }
}

// NOT WORKING
pub async fn speedgame(runtype: u8) -> anyhow::Result<()>//(String, String) // anyhow::Result<(String, String)>
{
    println!("START");
    let API_KEY = env::var("GIANTBOMB_API_KEY").context("missing GIANTBOMB_API_KEY environment variable")?;
    let offset = rand::thread_rng().gen_range(0..1500);
    let req_str = format!("http://www.giantbomb.com/api/games?api_key={}&format=json&platforms={}&limit=1&offset={}", API_KEY, 9.to_string(), offset.to_string());
    let resp = reqwest::get(req_str)
    .await?
    .json::<HashMap<String, String>>()
    .await?;
    println!("{:#?}", resp);
    //return (resp, "".to_string());

    Ok(())
    //let body = reqwest::get().await?.text().await?;
    //return body
}