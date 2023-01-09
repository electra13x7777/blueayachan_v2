///////////////////////////////////////////////////////////////////////////////
//                     EXTERNAL API COMMAND IMPLEMENTATIONS                  //
///////////////////////////////////////////////////////////////////////////////
use anyhow::Context;
use std::
{
    env,
    collections::HashMap,
};
use rand::Rng;
use chrono::NaiveDateTime;
use twitch_irc::
{
    message::{PrivmsgMessage},
};

use crate::db_ops::*;
use crate::models::*;
use serde::{Deserialize};

use serde_json::{Value};

// Comamand: !speedgame
//
//
pub async fn query_srl(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            let page_num: i32 = rand::thread_rng().gen_range(1..=6576);
            let req_str = format!("https://www.speedrunslive.com/api/games?pageNumber={}&pageSize=1", page_num);
            let data = reqwest::get(req_str).await?.text().await?;
            let results: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
            let game: String = format!("{}", &results["data"][0]["gameName"]);
            return Ok(format!("{} your new speedgame is {}!", msg_ctx.sender.name, game.replace('\"', "")));
        },
        b'?' =>
        {
            Ok("This command queries a random speedgame using SRL\'s API. TOTAL_GAMES: 6576".to_string())
        },
        b'#' =>
        {
            let page_num: i32 = rand::thread_rng().gen_range(1..=6576);
            let req_str = format!("https://www.speedrunslive.com/api/games?pageNumber={}&pageSize=1", page_num);
            let data = reqwest::get(req_str).await?.text().await?; // GET JaSON from
            let results: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
            let game: String = format!("{}", &results["data"][0]["gameName"]);
            let mut pop_string: String = format!("{}", &results["data"][0]["gamePopularity"]);
            pop_string = pop_string.replace('\"', "");
            let pop: f32 = pop_string.parse::<f32>().unwrap();
            let tenshi_quote: &str =
            if pop == 0.0{"Wow... no one plays this sh*t..."}
            else if pop >= 100.0{"...insane popularity! CirnoGenius 🤝 SomaCruzFromAriaOfSorrow"}
            else if pop >= 20.0{"Wow so popular! DataFace b"}
            else if pop < 20.0{"Holy cow someone has played this game!"}
            else{"Wow... no one plays this sh*t..."};
            return Ok(format!("{} your new speedgame is {}! Its popularity rating on SRL is {} TenshiWow o O ( {} ) ", msg_ctx.sender.name, game.replace('\"', ""), pop, tenshi_quote));
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
// Function: query_safebooru
// Return Type: Result<String>
// Description: Sends a GET request to the safebooru API which returns XML data for posts. Then parses that data into abstractions of the Posts on safebooru. 
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
        if !badges.contains(&"moderator".to_string()) && !badges.contains(&"broadcaster".to_string()) && !badges.contains(&"vip".to_string())
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
            let (_name, args) = match text.split_once(' ')
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
            Ok("This command queries an image from Safebooru. Use '*' to autocomplete a tag, a '+' to add an additional tag(s) to query with, or '-' to omit a tag from the search. | USAGE: !pic, !pic TAG, !pic TAG1+TAG2, !pic TAG1+...+TAGn, !pic TAG1+TAG2+-TAG3 | !pic shadow_h*from_*world+j*garland -> TAG1 = shadow_hearts_from_the_new_world, TAG2 = johnny_garland".to_string())
        },
        b'#' =>
        {
            let text = msg_ctx.message_text.as_str(); // get str from msg context
            let (_name, args) = match text.split_once(' ')
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
            extern crate rustnao;
            use rustnao::{Handler, HandlerBuilder, Sauce, Result};
            let sauce_handle = HandlerBuilder::new().api_key(&env::var("SAUCENAO_API_KEY").context("missing SAUCENAO_API_KEY environment variable")?).db_mask([Handler::PIXIV, Handler::DANBOORU].to_vec()).num_results(1).build();
            sauce_handle.set_min_similarity(45);
            let result : Result<Vec<Sauce>> = sauce_handle.get_sauce(&posts.post[index].file_url, None, None);
            println!("RES: \n{:#?}", result.unwrap()[0]);
            Ok(posts.post[index].file_url.to_owned())
        },
        _ => Ok(String::from("")),
    }
}