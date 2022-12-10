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
pub type Callback = fn(u8, PrivmsgMessage) -> String; //anyhow::Result<String>;// Option<String>;

pub struct EventHandler
{
    pub bot_nick: String,
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
        //print!("{}\n", name);
        if self.command_map.contains_key(&name)
        {
            handle_bac_user_in_db(msg.sender.name.clone()); // Updates user database
            const COMMAND_INDEX: usize = 0;
            let runtype: u8 = msg.message_text.clone().as_bytes()[COMMAND_INDEX]; // gets a byte literal (Ex. b'!')
            let out = self.command_map.get(&name).expect("Some shit went wrong!");
            let res = String::from(out(runtype, msg.clone()));
            let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
            println!("[{}] #{} <{}>: {}", dt_fmt, msg.channel_login, self.bot_nick, res);
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
            return format!("{}", String::from(args_start));
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
            else if rarity_weight >= 80
            {
                4
            }
            else if rarity_weight >= 60
            {
                3
            }
            else if rarity_weight >= 35
            {
                2
            }
            else
            {
                1
            };

            // HANDLE AUX DB STUFF
            // TODO: CHANGE THIS TO QUERY BY ID
            let bacuser: BACUser = query_user_data(msg_ctx.sender.name.to_lowercase());
            handle_user_last_demon(bacuser, &demon, &rarity);
            return String::from(format!("{} summoned a {}⭐ {}! {}", msg_ctx.sender.name, rarity, demon.demon_name, demon.demon_img_link));
        },
        b'?' =>
        {

            return String::from(format!("This command summons a random demon from Shin Megami Tensei III: Nocturne. Use <!savedemon> to save your last demon. Use <#demongacha> to see your saved demon. TOTAL_DEMONS: {}", get_demon_count()));
        },
        b'#' =>
        {
            // TODO: need to check for no table entry
            let bacuser: BACUser = query_user_data(msg_ctx.sender.name.to_lowercase());
            let sud = match query_user_demon(&bacuser)
            {
                // HANDLE SOME (GOOD DATA)
                Some(sud) => sud,
                // HANDLE NONE (NO DATA)
                None =>
                {
                    return format!("NO SAVED DEMON FOUND!!! {} please run <!demongacha> and <!savedemon> first!!!", msg_ctx.sender.name);
                },
            };
            let demon: NDemon = query_demon(sud.saved_demon_id);
            return format!("{} has a {}⭐ {}! {}", bacuser.user_nick, sud.saved_demon_rarity, demon.demon_name, demon.demon_img_link);

        },
        _ =>
        {
            return String::from("");
        },
    }
}

pub fn savedemon(runtype: u8, msg_ctx: PrivmsgMessage) -> String
{
    //const TOTAL_TWEETS: usize = 6569;
    match runtype
    {
        b'!' =>
        {
            let bacuser: BACUser = query_user_data(msg_ctx.sender.name.to_lowercase());
            save_user_demon(bacuser);
            return String::from(format!("{} saved their last demon", msg_ctx.sender.name));
        },
        b'?' =>
        {

            return String::from("This command saves the last demon you summoned with !demongacha.");
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
            return String::from(format!("{} rolled a {}⭐ {}!", msg_ctx.sender.name, rarity, ha));
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
            if user_data.num_commands == 1
            {
                return format!("{} has used {} command!", msg_ctx.sender.name, user_data.num_commands);
            }
            else
            {
                return format!("{} has used commands {} times!", msg_ctx.sender.name, user_data.num_commands);
            }
        },
        b'~' =>
        {
            return String::from("Test Tilde Block");
        },
        _ => {return String::from("");},
    }
}

pub fn melty(runtype: u8, msg_ctx: PrivmsgMessage) -> String
{
    match runtype
    {
        b'!' =>
        {
            let id: i32 = rand::thread_rng().gen_range(1..=get_melty_count()).try_into().unwrap();
            let queried_string: String = query_melty(id);
            let moonstyle_r: i8 = rand::thread_rng().gen_range(0..3);
            let moon: &str = match moonstyle_r
            {
                0 => "Crecent Moon",
                1 => "Half Moon",
                2 => "Full Moon",
                _ => "",
            };
            return format!("{} your new main in Melty Blood: Actress Again is {} {}!", msg_ctx.sender.name, moon.to_string(), queried_string);
        },
        b'?' =>
        {
            return format!("This command gives you a brand new main for Melty Blood: Actress Again");
        },
        _ => {return String::from("");},
    }
}

// SIMPLE GACHA COMMANDS
macro_rules! generate_simple_gacha
{
    ($fn_name:ident, $game_name:literal, $count:ident, $query_fn:ident) =>
    {
        pub fn $fn_name(runtype: u8, msg_ctx: PrivmsgMessage) -> String
        {
            match runtype
            {
                b'!' =>
                {
                    let id: i32 = rand::thread_rng().gen_range(1..=$count()).try_into().unwrap();
                    let queried_string: String = $query_fn(id);
                    return format!("{} your new main in {} is {}!", msg_ctx.sender.name, $game_name, queried_string);
                },
                b'?' =>
                {
                    return format!("This command gives you a brand new main for {}", $game_name);
                },
                _ => {return String::from("");},
            }
        }
    };
}
generate_simple_gacha!(lumina, "Melty Blood: Type Lumina", get_lumina_count, query_lumina);
generate_simple_gacha!(melee, "Super Smash Bros. Melee", get_melee_count, query_melee);
generate_simple_gacha!(soku, "Touhou 12.3: Hisoutensoku", get_soku_count, query_soku);
generate_simple_gacha!(bbcf, "BlazBlue Centralfiction", get_bbcf_count, query_bbcf);
generate_simple_gacha!(ggxxacplusr, "Guilty Gear XX Accent Core Plus R", get_ggxxacplusr_count, query_ggxxacplusr);
generate_simple_gacha!(akb, "Akatsuki Blitzkampf Ausf. Achse", get_akb_count, query_akb);
generate_simple_gacha!(vsav, "Vampire Savior: The Lord of Vampire", get_vsav_count, query_vsav);

// SIMPLE STRING COMMANDS
// a simple command is a command that generates the same text string every time
macro_rules! generate_simple_command
{
    ($fn_name:ident, $text:literal) =>
    {
        pub fn $fn_name(runtype: u8, msg_ctx: PrivmsgMessage) -> String
        {
            match runtype
            {
                b'!' =>
                {
                    return format!($text);
                }
                _ => {return String::from("");},
            }
        }
    };
}
generate_simple_command!(cmds, "Current Commands: dreamboumtweet, demongacha, savedemon, hornedanimegacha, melty, lumina, melee, soku, bbcf, ggxxacplusr, akb, vsav, me, help, cmds");
generate_simple_command!(help, "Blueayachan version 2 supports multiple different \"runtype\" characters : \'!\' is supposed to produce similar functionality to the previous bot. \'?\' should give information and help regarding that command. \'#\' does the standard command with different functionality that is specific to the command itself. for a list of commands type !cmds");
generate_simple_command!(poll, "THERE'S STILL TIME TO VOTE IN THE POLL! http://bombch.us/DYOt CirnoGenius");


// EXTERNAL QUERIES

/*
pub fn speedgame(runtype: u8, msg_ctx: PrivmsgMessage) -> String
{
    match runtype
    {
        b'!' =>
        {
            let game: String = speedgame_async().await.unwrap();
            return format!("{} your new speedgame is {}!", msg_ctx.sender.name, game);
        }
        _ =>{format!("")}
    }
    /*
    use rand::Rng;
    let page_num: i32 = rand::thread_rng().gen_range(1..=6576).try_into().unwrap();//100;
    let req_str = format!("https://www.speedrunslive.com/api/games?pageNumber={}&pageSize=1", page_num);
    let data = reqwest::blocking::get(req_str).expect("")
    //.await?
    .text().expect("");
    //.await?;
    use serde_json::{Result, Value};
    let mut results: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
    println!("{}", results["data"][0]["gameName"]);
    return format!("{} your new speedgame is {}!", msg_ctx.sender.name, results["data"][0]["gameName"]);*/
}

async fn speedgame_async() -> anyhow::Result<()>
{
    // REFERENCE: https://zint.ch/2022/01/14/unofficial-srl-api-docs.html#get-apigames
    //?sortBy=popularity
    use rand::Rng;
    let page_num: i32 = rand::thread_rng().gen_range(1..=6576).try_into().unwrap();//100;
    let req_str = format!("https://www.speedrunslive.com/api/games?pageNumber={}&pageSize=1", page_num);
    let data = reqwest::get(req_str)
    .await?
    .text()
    .await?;
    use serde_json::{Result, Value};
    //let v: Value = serde_json::from_str(data.as_str())?;
    let mut results: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
    println!("{}", results["data"][0]["gameName"]);
    /*body = "{\"data\":[{\"gameName\":\"The Great Waldo Search (NES)\",\"gameAbbr
    ev\":\"waldosearchnes\",\"gamePopularity\":0.000000,\"isSeasonGame\":false}]
    ,\"totalPages\":6576,\"pageNumber\":917,\"pageSize\":1}"*/
    //format!("{} your new speedgame is {}!", msg_ctx.sender.name, results["data"][0]["gameName"])
    Ok((results["data"][0]["gameName"].to_string()));
}
*/