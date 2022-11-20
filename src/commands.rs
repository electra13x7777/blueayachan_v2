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
use chrono;
use twitch_irc::
{
    login::StaticLoginCredentials,
    message::{PrivmsgMessage, ServerMessage},
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

pub type Twitch_Client = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;

/*
    COMMAND METHODOLOGY
        - Each command has 2 different ouputs
        - '!' before a string will attempt to execute the command
        - '?' before a string will attempt to help the user use the command

    name will be a Unique ID -> Value {Dataset, Help message}
*/
pub async fn execute_command(name: String, client: Twitch_Client, msg: PrivmsgMessage, cmd_map: HashMap<&str, (String, String)/*Box<Command>*/>) -> anyhow::Result<()>
{
    let command_index: usize = 0;
    let runtype_u8: u8 = msg.message_text.as_bytes()[command_index];
    let runtype: char = runtype_u8 as char;
    if cmd_map.contains_key(&name as &str)
    {
        let out = cmd_map.get(&name as &str).expect("Some shit went wrong!");
        match runtype
        {
            // Command Block
            '!' =>
            {
                let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
                println!("[{}] #{} <blueayachan>: {}", dt_fmt, msg.channel_login, out.0);
                client.say(
                        msg.channel_login.clone(),
                format!("{}", out.0)
                ).await?;
            },
            // Help Block
            '?' =>
            {
                let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
                println!("[{}] #{} <blueayachan>: {}", dt_fmt, msg.channel_login, out.1);
                client.say(
                        msg.channel_login.clone(),
                format!("{}", out.1),
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
pub fn dreamboumtweet() -> (String, String)
{
    fn readlines_to_vec(filename: impl AsRef<Path>) -> io::Result<Vec<String>>
    {
        BufReader::new(File::open(filename)?).lines().collect()
    }
    // read in list of tweets
    let mut dbt_vec = readlines_to_vec("dreamboum_tweets_10_05_2022.txt").expect("Could not load lines");
    let index = rand::thread_rng().gen_range(0..dbt_vec.len());
    // create temporary file to write html to
    let splitpoint: usize = 13;
    let length = dbt_vec[index].len();
    let tweet_ctx: &str = &dbt_vec[index][0..length-splitpoint];
    //let date_str: &str = &dbt_vec[index][length-splitpoint..];
    return (tweet_ctx.to_string(), "This command randomly sends a tweet made by twitter user @Dreamboum".to_string());
}

