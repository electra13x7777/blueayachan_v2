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
use crate::db_ops::*;
use crate::models::*;

pub type Twitch_Client = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;
// uses an unsigned 8bit int to signify what block to execute
pub type Callback = fn(u8, PrivmsgMessage) -> String;// Option<String>;

pub struct EventHandler
{
    pub bot_username: String,
    pub command_map: HashMap<String, Callback>,
}

impl EventHandler
{
    pub fn add_command(&mut self, name: String, function: Callback)
    {
        self.command_map.insert(name, function);
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
    pub async fn execute_command(&self, name: String, client: Twitch_Client, msg: PrivmsgMessage) -> anyhow::Result<()>
    {
        print!("{}\n", name);
        if self.command_map.contains_key(&name)
        {
            handle_bac_user_in_db(msg.sender.name.clone()); // Updates user database
            const COMMAND_INDEX: usize = 0;
            let runtype: u8 = msg.message_text.clone().as_bytes()[COMMAND_INDEX]; // gets a byte literal (Ex. b'!')
            let out = self.command_map.get(&name).expect("Some shit went wrong!");
            let res = String::from(out(runtype, msg.clone()));
            let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
            println!("[{}] #{} <{}>: {}", dt_fmt, msg.channel_login, self.bot_username, res);
            client.say(
                    msg.channel_login.clone(),
            format!("{}", res)
            ).await?;
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

pub fn test_command(runtype: u8, msg_ctx: PrivmsgMessage) -> String
{
    match runtype
    {
        b'!' =>
        {
            return String::from("Test Command Block");
        },
        b'?' =>
        {
            return String::from("Test Help Block");
        },
        b'#' =>
        {
            return String::from("Test Hash Block");
        },
        b'~' =>
        {
            return String::from("Test Tilde Block");
        },
        _ => {return String::from("");},
    }
}

// TODO: Stop being an idiot, learn how to parse well in this damn language
pub fn test_args(runtype: u8, msg_ctx: PrivmsgMessage) -> String
{
    print!("Executing\n");
    let text = msg_ctx.message_text.as_str(); // get str from msg context
    let (name, args_start) = match text.split_once(' ')
    {
        Some((name, args_start)) => (name, args_start),
        None => (text, ""),
    };
    //let args = text[args_start..];
    println!("{}", args_start);
    match runtype
    {
        b'!' =>
        {
            return format!("{} is a bitch", String::from(args_start));
        },
        _ => {return String::from("Uh oh");},
    }
}

// Preliminary implementation of dreamboumtweet (will eventually change)

pub fn dreamboumtweet(runtype: u8, msg_ctx: PrivmsgMessage) -> String//Option<String>//(String, String)
{
    //const TOTAL_TWEETS: usize = 6569;
    match runtype
    {
        b'!' =>
        {
            let id: i32 = rand::thread_rng().gen_range(1..=get_dbt_count()).try_into().unwrap();
            let tweet_ctx = query_single_dbtweet(id);
            return String::from(tweet_ctx);
        },
        b'?' =>
        {

            return String::from(format!("This command sends a random tweet made by twitter user @Dreamboum. TOTAL_TWEETS: {}", get_dbt_count()));
        },
        b'#' =>
        {
            let dbt_vec = readlines_to_vec("assets/dreamboum_tweets_10_05_2022.txt").expect("Could not load lines");
            let index = rand::thread_rng().gen_range(0..dbt_vec.len());
            let splitpoint: usize = 13;
            let length = dbt_vec[index].len();
            let tweet_ctx: &str = &dbt_vec[index];
            return String::from(tweet_ctx);
        },
        b'~' =>
        {
            let dbt_vec: Vec<(String, String)> = query_dbtweet_to_vec();
            let index = rand::thread_rng().gen_range(1..=dbt_vec.len());
            let tweet_ctx =  &dbt_vec[index].0;
            //let date_ctx = &dbt_vec[index].1;
            return String::from(tweet_ctx);
        },
        _ =>
        {
            return String::from("");
        },
    }
}
/*
macro_rules! generate_gacha_command {
    ($fn_name:ident,
    $runtype:u8,
    $new_struct_t:ident,
    $struct_t:ident
    $db_name:ident,) => {
        pub fn
    };
}*/

// DEMONGACHA
pub fn demongacha(runtype: u8, msg_ctx: PrivmsgMessage) -> String
{
    //const TOTAL_TWEETS: usize = 6569;
    match runtype
    {
        b'!' =>
        {
            // query random demon
            let id: i32 = rand::thread_rng().gen_range(1..=get_demon_count()).try_into().unwrap();
            let demon: NDemon = query_demon(id);
            // get rarity
            let rarity_weight: i8 = rand::thread_rng().gen_range(0..=100);
            let rarity =
            if rarity_weight>=95
            {
                5
            }
            else if rarity_weight >= 80 //&& rarity_weight < 95
            {
                4
            }
            else if rarity_weight >= 60 //&& rarity_weight < 80
            {
                3
            }
            else if rarity_weight >= 35 //&& rarity_weight < 60
            {
                2
            }
            else
            {
                1
            };
            return String::from(format!("{} summoned a {}⭐ {} {}", msg_ctx.sender.name, rarity, demon.demon_name, demon.demon_img_link));
        },
        b'?' =>
        {

            return String::from(format!("This command summons a random demon from Shin Megami Tensei III: Nocturne. TOTAL_DEMONS: {}", get_demon_count()));
        },
        _ =>
        {
            return String::from("");
        },
    }
}

pub fn hornedanimegacha(runtype: u8, msg_ctx: PrivmsgMessage) -> String
{
    //const TOTAL_TWEETS: usize = 6569;
    match runtype
    {
        b'!' =>
        {
            // query random demon
            let id: i32 = rand::thread_rng().gen_range(1..=get_hornedanime_count()).try_into().unwrap();
            let ha: String = query_hornedanime(id);
            // get rarity
            let rarity_weight: i8 = rand::thread_rng().gen_range(0..=100);
            let rarity =
            if rarity_weight>=95
            {
                5
            }
            else if rarity_weight >= 80 //&& rarity_weight < 95
            {
                4
            }
            else if rarity_weight >= 60 //&& rarity_weight < 80
            {
                3
            }
            else if rarity_weight >= 35 //&& rarity_weight < 60
            {
                2
            }
            else
            {
                1
            };
            return String::from(format!("{} rolled a {}⭐ {}", msg_ctx.sender.name, rarity, ha));
        },
        b'?' =>
        {

            return String::from(format!("This command rolls for a random HornedAnime. TOTAL_HORNEDANIMES: {}", get_hornedanime_count()));
        },
        _ =>
        {
            return String::from("");
        },
    }
}

pub fn me(runtype: u8, msg_ctx: PrivmsgMessage) -> String
{
    let user_data: BACUser = query_user_data(msg_ctx.sender.name.to_lowercase());
    match runtype
    {
        b'!' =>
        {
            return format!("| Nick: {} | Commands: {} | Date Added: {} |", msg_ctx.sender.name, user_data.num_commands, user_data.date_added);
        },
        b'?' =>
        {
            return format!("BAC User {} added on {}", msg_ctx.sender.name, user_data.date_added);
        },
        b'#' =>
        {
            return format!("{} has used commands {} times!", msg_ctx.sender.name, user_data.num_commands);
        },
        b'~' =>
        {
            return String::from("Test Tilde Block");
        },
        _ => {return String::from("");},
    }
}