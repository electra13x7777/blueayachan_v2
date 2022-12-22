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
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};
use serde_json::{Result, Value};
use crate::helpers::readlines_to_vec;
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
    */
    pub async fn execute_command(&self, name: String, client: Twitch_Client, msg: PrivmsgMessage) -> anyhow::Result<()>
    {
        if self.command_map.contains_key(&name)
        {
            // TODO: check if command is allowed in channel
            handle_bac_user_in_db(msg.sender.name.clone()); // Updates user database
            const COMMAND_INDEX: usize = 0;
            let runtype: u8 = msg.message_text.clone().as_bytes()[COMMAND_INDEX]; // gets a byte literal (Ex. b'!')
            let callback = self.command_map.get(&name).expect("Could not execute function pointer!");
            let res = String::from(callback(runtype, msg.clone()).await.unwrap());
            if &res == ""{return Ok(());} // if we have nothing to send skip the send
            let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
            const COLOR_FLAG: bool = true;
            match COLOR_FLAG
            {
                true => println!("[{}] #{} <{}>: {}", dt_fmt.truecolor(138, 138, 138), msg.channel_login.truecolor(117, 97, 158), self.bot_nick.red(), res),
                false => println!("[{}] #{} <{}>: {}", dt_fmt, msg.channel_login, self.bot_nick, res),
            }
            client.say(msg.channel_login.clone(), format!("{}", res)).await?;
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
        _ => Ok(String::from("")),
    }
}

pub async fn dreamboumtweet(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>//Option<String>//(String, String)
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
pub async fn demongacha(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
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

pub async fn savedemon(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
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


pub async fn hornedanimegacha(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
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

pub async fn melty(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
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
        _ => Ok(String::from("")),
    }
}

pub async fn kinohackers(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            let id: i32 = rand::thread_rng().gen_range(1..=get_kinohack_count()).try_into().unwrap();
            let queried_link: String = query_kinohackers(id);
            return Ok(format!("{}", queried_link));
        },
        b'?' =>
        {
            return Ok(format!("This command gives you a brand kinohackers meme made by various members of the Claude influencer circle"));
        },
        _ => Ok(String::from("")),
    }
}
// refactor into a commands/misc dir
pub async fn shftnw(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            return Ok(format!("{} loves Shadow Hearts: From the New World!", msg_ctx.sender.name));
        },
        b'?' =>
        {
            return Ok(format!("This is the worlds most useless command"));
        },
        _ => Ok(String::from("")),
    }
}


// SIMPLE GACHA COMMANDS
macro_rules! generate_simple_gacha
{
    ($fn_name:ident, $game_name:literal, $count:ident, $query_fn:ident) =>
    {
        pub async fn $fn_name(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
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
                _ => Ok(String::from("")),
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
        pub async fn $fn_name(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
        {
            match runtype
            {
                b'!' =>
                {
                    return Ok(format!($text));
                }
                _ => Ok(String::from("")),
            }
        }
    };
}
generate_simple_command!(cmds, "Current Commands: dreamboumtweet, demongacha, savedemon, hornedanimegacha, chen, speedgame, pic, pick, range, hentai, kinohackers, melty, lumina, melee, soku, bbcf, ggxxacplusr, akb, vsav, jojos, millions, cfb, me, help, cmds, repo, weekly");
generate_simple_command!(help, "Blueayachan version 2 supports multiple different \"runtype\" characters : \'!\' is supposed to produce similar functionality to the previous bot. \'?\' should give information and help regarding that command. \'#\' does the standard command with different functionality that is specific to the command itself. For a list of commands type !cmds");
generate_simple_command!(poll, "THERE'S STILL TIME TO VOTE IN THE POLL! http://bombch.us/DYOt CirnoGenius");
generate_simple_command!(repo, "You can find the github repository here: https://github.com/electra13x7777/blueayachan_v2");
generate_simple_command!(weekly, "Last Week's Top 15: https://imgur.com/a/PYmokTp");


///////////////////////////////////////////////////////////////////////////////
//                    AUX CHAT HELP COMMAND IMPLEMENTATIONS                  //
///////////////////////////////////////////////////////////////////////////////

pub async fn me(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    let user_data: BACUser = query_user_data(msg_ctx.sender.name.to_lowercase());
    match runtype
    {
        b'!' =>
        {
            let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
            let days = ndt_now.signed_duration_since(user_data.date_added).num_days();
            //return Ok(format!("| Nick: {} | Commands: {} | Date Added: {} |", msg_ctx.sender.name, user_data.num_commands, user_data.date_added));
            match days
            {
                0 => return Ok(format!("{} became a BAC user today! They have used {} commands.", msg_ctx.sender.name, user_data.num_commands)),
                1 => return Ok(format!("{} has been a BAC user for {} day. They have used {} commands.", msg_ctx.sender.name, days, user_data.num_commands)),
                _ => return Ok(format!("{} has been a BAC user for {} days. They have used {} commands.", msg_ctx.sender.name, days, user_data.num_commands))
            }
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
        _ => Ok(String::from("")),
    }
}

// CURRENTLY DISABLED DUE TO INPUT VALIDATION BUGS
pub async fn range(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    fn arg_is_int(s: &String) -> bool
    {
        let mut i: i32 = 0;
        for c in s.chars()
        {
            if !c.is_numeric() && c != '-'
            {
                return false;
            }
            else if s.len() == 1 && !c.is_numeric()
            {
                return false;
            }
            else if i != 0 && !c.is_numeric()
            {
                return false;
            }
            i+=1;
        }
        return true;
    }
    match runtype
    {
        b'!' =>
        {
            const I64LEN: usize = "9223372036854775807".len();
            let text = msg_ctx.message_text.as_str();
            let (name, args) = match text.split_once(' ')
            {
                Some((name, args)) => (name, args),
                None => (text, ""),
            };

            let argv_s: Vec<String> = args.split(' ').map(|s| s.to_string()).collect();
            // check arg count
            if argv_s.len() != 2{return Ok(format!("Bad argument count! Please make sure your command follows this syntax: !range INT1 INT2"));}
            // check if int
            if !arg_is_int(&argv_s[0]) || !arg_is_int(&argv_s[1]){return Ok(format!("Bad argument found! Please make sure you are providing INTEGERS as arguments. Ex) 1000, -500, 69, -420"));}
            // check input string length
            if argv_s[0].len() >= I64LEN || argv_s[1].len() >= I64LEN
            {
                return Ok(format!("One or more of the arguments provided are not only above 32 bits, they are also above max signed 64bit integer bounds..."));
            }
            let mut argv: Vec<i64> = vec![argv_s[0].parse::<i64>().unwrap(), argv_s[1].parse::<i64>().unwrap()];
            if argv[0] >= i32::MAX.into() || argv[0] <= i32::MIN.into()
            {
                return Ok(format!("Make sure the first argument provided is no greater than {} and no less than {}", i32::MAX, i32::MIN));
            }
            if argv[1] >= i32::MAX.into() || argv[1] <= i32::MIN.into()
            {
                return Ok(format!("Make sure the second argument provided is no greater than {} and no less than {}", i32::MAX, i32::MIN));
            }

            if argv[0] > argv[1]{argv.swap(0, 1);}
            let rand_int: i64 = rand::thread_rng().gen_range(argv[0]..=argv[1]);
            Ok(format!("{} your new integer value is {}!", msg_ctx.sender.name, rand_int))
        },
        b'?' =>
        {
            Ok(format!("This command picks a random 32 bit integer in a given range. Use whitespace to separate the numbers. | USAGE: !range INT1 INT2 | !range INT2 INT1 -> swaps larger and smaller to make it easy to use. NOTE: Range command is INCLUSIVE of the upperbound"))
        },
        _ => Ok(String::from("")),
    }
}

pub async fn pick(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
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
            let argv: Vec<String> = args.split(' ').map(|s| s.to_string()).collect();
            //args.split_whitespace().collect();//::<Vec<String>>();//.join("+")
            let index: usize = rand::thread_rng().gen_range(0..argv.len());
            Ok(format!("picks {}", argv[index]))

        },
        b'?' =>
        {
            Ok(format!("This command picks a single argument from input provided via message. Use whitespace to make another argument for the bot to pick from (will be better in the future) | USAGE: !pick, !pick ARG, !pick ARG1 ARG2, !pick ARG1 ... ARGn |"))
        },
        _ => Ok(String::from("")),
    }
}

pub async fn is_hentai(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            let out: Vec<&str> = vec!("This game is hentai DataSweat", "This game is NOT hentai YoumuAngry", "This game could possibly be hentai, but more testing is needed MarisaFace");
            let index: usize = rand::thread_rng().gen_range(0..out.len());
            Ok(format!("{}", out[index]))

        },
        b'?' =>
        {
            Ok(format!("This command lets the bot decide if any content on the stream contains hentai. NOTE: The author of this command does not guarantee its reliability..."))
        },
        _ => Ok(String::from("")),
    }
}

pub async fn chen(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            let mut chen_str: (String, String) = match msg_ctx.channel_login.as_str() // will read from a database eventually
            {
                "claude" => (String::from("HONKHONK"), String::from("CirnoGenius")),
                "darko_rta" => (String::from("saHonk"), String::from("CirnoGenius")),
                "electra_rta" => (String::from("saHonk"), String::from("CirnoGenius")),
                "crypton42" => (String::from("saHonk"), String::from("saHonk")),
                _ => (String::from(""), String::from(""))
            };
            if chen_str.0 == ""{return Ok(String::from(""))}
            let chens: usize = rand::thread_rng().gen_range(0..=10);
            match chens
            {
                0 => return Ok(format!("{}... got 0 chens :(", msg_ctx.sender.name)),
                1 => return Ok(format!("{} got {} chen. {}",  msg_ctx.sender.name, chens, chen_str.0)),
                _ => chens
            };
            let mut new_chens: String = "".to_owned();
            for i in 1..=chens
            {
                match chens
                {
                    9 => new_chens += &chen_str.1,
                    _ => new_chens += &chen_str.0
                }

                if i != chens
                {
                    new_chens += " ";
                }
            }
            Ok(format!("{} got a {} chen combo! {}",  msg_ctx.sender.name, chens, new_chens))
        },
        b'?' =>
        {
            Ok(format!("This command gives you chens. Yay!"))
        },
        b'#' =>
        {
            let chen_str: String = match msg_ctx.channel_login.as_str() // will read from a database eventually
            {
                "claude" => String::from("HONKHONK"),
                "darko_rta" => String::from("saHonk"),
                "electra_rta" => String::from("saHonk"),
                "crypton42" => String::from("saHonk"),
                _ => String::from("")
            };
            if chen_str == ""{return Ok(String::from(""))}
            let chens: usize = rand::thread_rng().gen_range(0..=10);
            match chens
            {
                0 => return Ok(format!("0 chens :(")),
                1 => return Ok(format!("{}", chen_str)),
                _ => chens
            };
            let mut new_chens: String = "".to_owned();
            for i in 1..=chens
            {
                new_chens += &chen_str;
                if i != chens
                {
                    new_chens += " ";
                }
            }
            Ok(format!("{}", new_chens))
        },
        _ => Ok(String::from("")),
    }
}

pub async fn cfb(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            let c_list: Vec<String> = readlines_to_vec("assets/c.txt").expect("Error reading file!");
            let f_list: Vec<String> = readlines_to_vec("assets/f.txt").expect("Error reading file!");
            let b_list: Vec<String> = readlines_to_vec("assets/b.txt").expect("Error reading file!");
            let index_c: usize = rand::thread_rng().gen_range(0..c_list.len());
            let index_f: usize = rand::thread_rng().gen_range(0..f_list.len());
            let index_b: usize = rand::thread_rng().gen_range(0..b_list.len());
            Ok(format!("{} {} {}", c_list[index_c], f_list[index_f], b_list[index_b]))
        },
        b'?' =>
        {
            Ok(format!("This command generates a string containing words that start with C, F, and B"))
        },
        _ => Ok(String::from("")),
    }
}

///////////////////////////////////////////////////////////////////////////////
//                     EXTERNAL API COMMAND IMPLEMENTATIONS                  //
///////////////////////////////////////////////////////////////////////////////

// Comamand: !speedgame
//
//
pub async fn query_srl(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
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
            let mut pop_string: String = format!("{}", &results["data"][0]["gamePopularity"]);
            pop_string = pop_string.replace("\"", "");
            let pop: f32 = pop_string.parse::<f32>().unwrap();
            let tenshi_quote: &str =
            if pop == 0.0{"Wow... no one plays this sh*t..."}
            else if pop >= 100.0{"...insane popularity! CirnoGenius 🤝 SomaCruzFromAriaOfSorrow"}
            else if pop >= 20.0{"Wow so popular! DataFace b"}
            else if pop < 20.0{"Holy cow someone has played this game!"}
            else{"Wow... no one plays this sh*t..."};
            return Ok(format!("{} your new speedgame is {}! Its popularity rating on SRL is {} TenshiWow o O ( {} ) ", msg_ctx.sender.name, game.replace("\"", ""), pop, tenshi_quote));
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

// Command: !pic
//
//
pub async fn query_safebooru(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    const HAS_TIMEOUT: bool = true;
    const CHANNEL_FILTER: bool = true;
    const FILTERED: &'static [&'static str] = &["sioneus", "cyghfer", "liquidsquid"]; // will read from a database eventually
    const MOD_ONLY: &'static [&'static str] = &["mpghappiness"];
    if CHANNEL_FILTER && FILTERED.contains(&msg_ctx.channel_login.as_str())
    {
        return Ok(format!("This command is not available in {}\'s channel. Sorry {}", msg_ctx.channel_login, msg_ctx.sender.name));
    }
    if CHANNEL_FILTER && MOD_ONLY.contains(&msg_ctx.channel_login.as_str())
    {
        let badges: Vec<String> = msg_ctx.badges.iter().map(|b| b.name.clone()).collect();
        if !badges.contains(&"moderator".to_string()) && !badges.contains(&"broadcaster".to_string())
        {
            return Ok(format!("This command is not available to non-mods in {}\'s channel. Sorry {}", msg_ctx.channel_login, msg_ctx.sender.name));
        }
    }
    const TIMEOUT_DIFF: i64 = 30;
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
            let req_str = format!("https://safebooru.org/index.php?page=dapi&s=post&q=index&rating=g&tags={}+-rating:questionable", &args.to_lowercase());
            let data = reqwest::get(req_str).await?.text().await?;
            let posts: SafebooruPosts = match serde_xml_rs::from_str(&data)
            {
                Ok(posts) => posts,
                _ => return Ok(format!("No results found for given arguments: {} https://imgur.com/a/vQsv7Rj", &args)),
            };
            // handle timeout when we know we have queried an image
            if HAS_TIMEOUT
            {
                let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
                let bacuser: BACUser = query_user_data(msg_ctx.sender.name.to_lowercase());
                let timeout_out: (bool, i64) = handle_pic_timeout(bacuser, ndt_now, TIMEOUT_DIFF);
                if !timeout_out.0 // User has not waited for the timeout length
                {
                    return Ok(format!("{}, please wait for {} more second(s)", msg_ctx.sender.name, TIMEOUT_DIFF - timeout_out.1))
                }
            }
            let index: usize = rand::thread_rng().gen_range(0..posts.post.len());
            Ok(posts.post[index].file_url.to_owned())
        },
        b'?' =>
        {
            Ok(format!("This command queries an image from Safebooru. Use '*' to autocomplete a tag, a '+' to add an additional tag(s) to query with, or '-' to omit a tag from the search. | USAGE: !pic, !pic TAG, !pic TAG1+TAG2, !pic TAG1+...+TAGn, !pic TAG1+TAG2+-TAG3 | !pic shadow_h*from_*world+j*garland -> TAG1 = shadow_hearts_from_the_new_world, TAG2 = johnny_garland"))
        },
        _ => Ok(String::from("")),
    }
}