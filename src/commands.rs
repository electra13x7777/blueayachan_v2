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
use colored::*;
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};
use serde_json::{Result, Value};
use crate::helpers::readlines_to_vec;
use crate::db_ops::*;
use crate::models::*;

pub type Twitch_Client = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;

// CALLBACK TYPES (Blocking, Non-Blocking) [Function Pointers]
pub type Callback = fn(u8, PrivmsgMessage) -> anyhow::Result<String>;
//pub type Callback_a = Box<dyn Fn(u8, PrivmsgMessage) -> Pin<Box<dyn Future<Output = anyhow::Result<String>>>> + 'static + Send + Sync,>;


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
impl EventHandler
{
    fn insert_command<Cb, F>(&mut self, name: String, function: Cb)
    where
    Cb: Fn(u8, PrivmsgMessage) -> F + 'static + Send + Sync,
    F: Future<Output = anyhow::Result<String>> + 'static + Send + Sync,
    {
        let cb = Box::new(move |a, b| Box::pin(function(a, b)) as _);
        self.command_map.insert(name, cb);
    }
}
*/
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
    */
    pub async fn execute_command(&self, name: String, client: Twitch_Client, msg: PrivmsgMessage) -> anyhow::Result<()>
    {
        if self.command_map.contains_key(&name)
        {
            handle_bac_user_in_db(msg.sender.name.clone()); // Updates user database
            const COMMAND_INDEX: usize = 0;
            let runtype: u8 = msg.message_text.clone().as_bytes()[COMMAND_INDEX]; // gets a byte literal (Ex. b'!')
            // TODO: Make this a special case for non blocking commands
            if self.command_map[&name] == async_placeholder // ASYNC COMMAND
            {
                // TODO: try to find a way to make a fn pointer for Futures
                let res = match &name[..]
                {
                    "speedgame" => query_srl(runtype, msg.clone()).await.unwrap(),
                    "pic" => query_safebooru(runtype, msg.clone()).await.unwrap(),
                    _ => return Ok(()),
                };
                let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
                const color_flag: bool = true;
                match color_flag
                {
                    true => println!("[{}] #{} <{}>: {}", dt_fmt.truecolor(138, 138, 138), msg.channel_login.truecolor(117, 97, 158), self.bot_nick.red(), res),
                    false => println!("[{}] #{} <{}>: {}", dt_fmt, msg.channel_login, self.bot_nick, res),
                }
                if &res == ""{return Ok(());} // if we have nothing to send skip the send
                client.say(
                        msg.channel_login.clone(),
                format!("{}", res)
                ).await?;
            }
            else // BLOCKING COMMAND
            {
                let out = self.command_map.get(&name).expect("Could not execute function pointer!");
                let res = String::from(out(runtype, msg.clone()).unwrap());
                let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
                const color_flag: bool = true;
                match color_flag
                {
                    true => println!("[{}] #{} <{}>: {}", dt_fmt.truecolor(138, 138, 138), msg.channel_login.truecolor(117, 97, 158), self.bot_nick.red(), res),
                    false => println!("[{}] #{} <{}>: {}", dt_fmt, msg.channel_login, self.bot_nick, res),
                }
                if &res == ""{return Ok(());} // if we have nothing to send skip the send
                client.say(
                        msg.channel_login.clone(),
                format!("{}", res)
                ).await?;
            }
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
        _ => {Ok(String::from(""))},
    }
}

pub fn dreamboumtweet(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>//Option<String>//(String, String)
{
    //const TOTAL_TWEETS: usize = 6569;
    match runtype
    {
        b'!' =>
        {
            let id: i32 = rand::thread_rng().gen_range(1..=get_dbt_count()).try_into().unwrap();
            let tweet_ctx = query_single_dbtweet(id);
            return Ok(String::from(tweet_ctx));
        },
        b'?' =>
        {
            return Ok(format!("This command sends a random tweet made by twitter user @Dreamboum. TOTAL_TWEETS: {}", get_dbt_count()));
        },
        b'#' =>
        {
            let dbt_vec = readlines_to_vec("assets/dreamboum_tweets_10_05_2022.txt").expect("Could not load lines");
            let index = rand::thread_rng().gen_range(0..dbt_vec.len());
            let splitpoint: usize = 13;
            let length = dbt_vec[index].len();
            let tweet_ctx: &str = &dbt_vec[index];
            return Ok(String::from(tweet_ctx));
        },
        b'~' =>
        {
            let dbt_vec: Vec<(String, String)> = query_dbtweet_to_vec();
            let index = rand::thread_rng().gen_range(1..=dbt_vec.len());
            let tweet_ctx =  &dbt_vec[index].0;
            //let date_ctx = &dbt_vec[index].1;
            return Ok(String::from(tweet_ctx));
        },
        _ =>
        {
            Ok(String::from(""))
        },
    }
}

// DEMONGACHA
pub fn demongacha(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    //const TOTAL_TWEETS: usize = 6569;
    match runtype
    {
        b'!' =>
        {
            // query random demon
            let id: i32 =rand::thread_rng().gen_range(1..=get_demon_count()).try_into().unwrap();
            //println!("{}", id);
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
            return Ok(format!("{} summoned a {}⭐ {}! {}", msg_ctx.sender.name, rarity, demon.demon_name, demon.demon_img_link));
        },
        b'?' =>
        {

            return Ok(format!("This command summons a random demon from Shin Megami Tensei III: Nocturne. Use <!savedemon> to save your last demon. Use <#demongacha> to see your saved demon. TOTAL_DEMONS: {}", get_demon_count()));
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
                    return Ok(format!("NO SAVED DEMON FOUND!!! {} please run <!demongacha> and <!savedemon> first!!!", msg_ctx.sender.name));
                },
            };
            let demon: NDemon = query_demon(sud.saved_demon_id);
            return Ok(format!("{} has a {}⭐ {}! {}", bacuser.user_nick, sud.saved_demon_rarity, demon.demon_name, demon.demon_img_link));

        },
        _ =>
        {
            Ok(String::from(""))
        },
    }
}

pub fn savedemon(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    //const TOTAL_TWEETS: usize = 6569;
    match runtype
    {
        b'!' =>
        {
            let bacuser: BACUser = query_user_data(msg_ctx.sender.name.to_lowercase());
            save_user_demon(bacuser);
            return Ok(format!("{} saved their last demon", msg_ctx.sender.name));
        },
        b'?' =>
        {

            return Ok(format!("This command saves the last demon you summoned with !demongacha."));
        },
        _ =>
        {
            Ok(String::from(""))
        },
    }
}


pub fn hornedanimegacha(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
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
            return Ok(format!("{} rolled a {}⭐ {}!", msg_ctx.sender.name, rarity, ha));
        },
        b'?' =>
        {

            return Ok(format!("This command rolls for a random HornedAnime. TOTAL_HORNEDANIMES: {}", get_hornedanime_count()));
        },
        _ =>
        {
            Ok(String::from(""))
        },
    }
}

pub fn me(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    let user_data: BACUser = query_user_data(msg_ctx.sender.name.to_lowercase());
    match runtype
    {
        b'!' =>
        {
            return Ok(format!("| Nick: {} | Commands: {} | Date Added: {} |", msg_ctx.sender.name, user_data.num_commands, user_data.date_added));
        },
        b'?' =>
        {
            return Ok(format!("This command returns information based on your usage of the bot."));
        },
        b'#' =>
        {
            if user_data.num_commands == 1
            {
                return Ok(format!("{} has used {} command!", msg_ctx.sender.name, user_data.num_commands));
            }
            else
            {
                return Ok(format!("{} has used commands {} times!", msg_ctx.sender.name, user_data.num_commands));
            }
        },
        _ => {Ok(String::from(""))},
    }
}

pub fn melty(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
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
                0 => "Crescent Moon",
                1 => "Half Moon",
                2 => "Full Moon",
                _ => "",
            };
            return Ok(format!("{} your new main in Melty Blood: Actress Again is {} {}!", msg_ctx.sender.name, moon.to_string(), queried_string));
        },
        b'?' =>
        {
            return Ok(format!("This command gives you a brand new main for Melty Blood: Actress Again"));
        },
        _ => {Ok(String::from(""))},
    }
}

// SIMPLE GACHA COMMANDS
macro_rules! generate_simple_gacha
{
    ($fn_name:ident, $game_name:literal, $count:ident, $query_fn:ident) =>
    {
        pub fn $fn_name(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
        {
            match runtype
            {
                b'!' =>
                {
                    let id: i32 = rand::thread_rng().gen_range(1..=$count()).try_into().unwrap();
                    let queried_string: String = $query_fn(id);
                    return Ok(format!("{} your new main in {} is {}!", msg_ctx.sender.name, $game_name, queried_string));
                },
                b'?' =>
                {
                    return Ok(format!("This command gives you a brand new main for {}", $game_name));
                },
                _ => {Ok(String::from(""))},
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
generate_simple_gacha!(jojos, "JoJo\'s Bizarre Adventure: Heritage for the Future", get_jojo_count, query_jojo);
generate_simple_gacha!(millions, "Million Arthur: Arcana Blood", get_millions_count, query_millions);

// SIMPLE STRING COMMANDS
// a simple command is a command that generates the same text string every time
macro_rules! generate_simple_command
{
    ($fn_name:ident, $text:literal) =>
    {
        pub fn $fn_name(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
        {
            match runtype
            {
                b'!' =>
                {
                    return Ok(format!($text));
                }
                _ => {Ok(String::from(""))},
            }
        }
    };
}
generate_simple_command!(cmds, "Current Commands: dreamboumtweet, demongacha, savedemon, hornedanimegacha, speedgame, pic, melty, lumina, melee, soku, bbcf, ggxxacplusr, akb, vsav, jojos, millions, me, help, cmds, repo");
generate_simple_command!(help, "Blueayachan version 2 supports multiple different \"runtype\" characters : \'!\' is supposed to produce similar functionality to the previous bot. \'?\' should give information and help regarding that command. \'#\' does the standard command with different functionality that is specific to the command itself. for a list of commands type !cmds");
generate_simple_command!(poll, "THERE'S STILL TIME TO VOTE IN THE POLL! http://bombch.us/DYOt CirnoGenius");
generate_simple_command!(repo, "You can find the githup repository here: https://github.com/electra13x7777/blueayachan_v2");


///////////////////////////////////////////////////////////////////////////////
//                     NON-BLOCKING COMMAND IMPLEMENTATIONS                  //
///////////////////////////////////////////////////////////////////////////////

// this is terrible design - placeholder function to map to async commands
pub fn async_placeholder(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    Ok(String::from("A Placeholder function"))
}

async fn query_srl(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            use rand::Rng;
            let page_num: i32 = rand::thread_rng().gen_range(1..=6576).try_into().unwrap();
            let req_str = format!("https://www.speedrunslive.com/api/games?pageNumber={}&pageSize=1", page_num);
            let data = reqwest::get(req_str).await?.text().await?;
            let mut results: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
            let mut game: String = format!("{}", &results["data"][0]["gameName"]);
            return Ok(format!("{} your new speedgame is {}!", msg_ctx.sender.name, game.replace("\"", "")));
        },
        b'?' =>
        {
            Ok(format!("This command queries a random speedgame using SRL\'s API. TOTAL_GAMES: 6576"))
        },
        b'#' =>
        {
            use rand::Rng;
            let page_num: i32 = rand::thread_rng().gen_range(1..=6576).try_into().unwrap();
            let req_str = format!("https://www.speedrunslive.com/api/games?pageNumber={}&pageSize=1", page_num);
            let data = reqwest::get(req_str).await?.text().await?; // GET JaSON from
            let mut results: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
            let mut game: String = format!("{}", &results["data"][0]["gameName"]);
            let mut pop: String = format!("{}", &results["data"][0]["gamePopularity"]);
            return Ok(format!("{} your new speedgame is {}! Its popularity rating on SRL is {} TenshiWow o O ( Wow so popular! ) ", msg_ctx.sender.name, game.replace("\"", ""), pop.replace("\"", "")));
        },
        _ => Ok(String::from("")),
    }
}


#[derive(Debug, Deserialize)]
pub struct SafebooruPost
{
    file_url: String,
}
#[derive(Debug, Deserialize)]
pub struct SafebooruPosts
{
    post: Vec<SafebooruPost>,
}

async fn query_safebooru(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            let text = msg_ctx.message_text.as_str(); // get str from msg context
            let (name, args) = match text.split_once(' ')
            {
                Some((name, args)) => (name, args),
                None => (text, ""),
            };
            // should we ever want to refactor to have whitespace split the 2 tag arguments
            //args.split_whitespace().collect::<Vec<_>>().join("+")
            let req_str = format!("https://safebooru.org/index.php?page=dapi&s=post&q=index&tags={}", &args);
            let data = reqwest::get(req_str).await?.text().await?;

            let posts: SafebooruPosts = match serde_xml_rs::from_str(&data)
            {
                Ok(posts) => posts,
                _ => return Ok(format!("No results found for given arguments: {}", &args)),
            };
            let index: usize = rand::thread_rng().gen_range(0..posts.post.len());
            Ok(posts.post[index].file_url.to_owned())
        },
        b'?' =>
        {
            Ok(format!("This command queries an image from Safebooru. Use '*' to autocomplete a tag and '+' to add an additional tag(s) to query with. | USAGE: !pic, !pic TAG, !pic TAG1+TAG2, !pic TAG1+...+TAGn | !pic shadow_h*from_*world+j*garland -> TAG1 = shadow_hearts_from_the_new_world, TAG2 = johnny_garland"))
        },
        _ => Ok(String::from("")),
    }
}