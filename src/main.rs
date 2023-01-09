#![allow(nonstandard_style)]
#![allow(clippy::needless_return, clippy::redundant_static_lifetimes)]
/*
    FILE: main.rs
    AUTHOR(s): azuchang, electra_rta

    SPECIAL THANKS: azuchang (https://github.com/Heliozoa) has generously provided
                the skeleton to this codebase. Please give them the credit they
                deserve!
*/

use anyhow::Context;
use std::
{
    env,
    collections::HashMap,
};

use twitch_irc::
{
    login::StaticLoginCredentials,
    message::
    {
        PrivmsgMessage,
        ServerMessage,
    },
    ClientConfig,
};
use colored::*;

pub mod commands;
use crate::commands::
{
    Twitch_Client,
    EventHandler,
};
pub mod cmds;


pub mod helpers;
use crate::helpers::readlines_to_vec;

pub mod db_connect;
pub mod models;
pub mod schema;
pub mod db_ops;
//pub mod test_db_stuff;
#[macro_use]
extern crate diesel;

//use crate::test_db_stuff::test;

#[tokio::main]
async fn main() -> anyhow::Result<()>
{
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();

    let bot_username =
    env::var("BOT_USERNAME").context("missing BOT_USERNAME environment variable")?;
    let oauth_token =
    env::var("OAUTH_TOKEN").context("missing OAUTH_TOKEN environment variable")?;
    // TODO: channels should be queried from a database
    //let channel = env::var("CHANNEL_NAME").context("missing CHANNEL_NAME environment variable")?;
    let channels = readlines_to_vec("assets/channels.txt").expect("Failed to read file");
    //let wink = env::var("WINK").context("missing CHANNEL_NAME environment variable")?;
    let aya_vec = readlines_to_vec("assets/ayawink.txt");
    let av_iter = aya_vec.iter();
    for line in av_iter
    {
        //let f_line = format!("{}\n", line);
        println!("{}", format!("{:#?}", line));
    }

    // TEMP SETUP COMMANDS
    let bot_nick: String = bot_username.clone();
    let mut handler = EventHandler { bot_nick, command_map: HashMap::new() };
    // TEST
    //handler.add_command(String::from("test"), commands::test_command);

    // GACHAS
    handler.add_command(String::from("dreamboumtweet"), cmds::cmd_gacha::dreamboumtweet);
    handler.add_command(String::from("demongacha"), cmds::cmd_gacha::demongacha);
    handler.add_command(String::from("savedemon"), cmds::cmd_gacha::savedemon);
    handler.add_command(String::from("hornedanimegacha"), cmds::cmd_gacha::hornedanimegacha);
    handler.add_command(String::from("chen"), cmds::cmd_gacha::chen);
    handler.add_command(String::from("melty"), cmds::cmd_gacha::melty);
    handler.add_command(String::from("lumina"), cmds::cmd_gacha::lumina);
    handler.add_command(String::from("melee"), cmds::cmd_gacha::melee);
    handler.add_command(String::from("soku"), cmds::cmd_gacha::soku);
    handler.add_command(String::from("bbcf"), cmds::cmd_gacha::bbcf);
    handler.add_command(String::from("ggxxacplusr"), cmds::cmd_gacha::ggxxacplusr);
    handler.add_command(String::from("akb"), cmds::cmd_gacha::akb);
    handler.add_command(String::from("vsav"), cmds::cmd_gacha::vsav);
    handler.add_command(String::from("jojos"), cmds::cmd_gacha::jojos);
    handler.add_command(String::from("millions"), cmds::cmd_gacha::millions);

    // USERINFO
    handler.add_command(String::from("me"), cmds::cmd_userinfo::me);

    // EXTERNAL GET REQUESTS
    handler.add_command(String::from("speedgame"), cmds::cmd_externalquery::query_srl);
    handler.add_command(String::from("pic"), cmds::cmd_externalquery::query_safebooru);

    // MISC COMMANDS
    handler.add_command(String::from("kinohackers"), cmds::cmd_misc::kinohackers);
    handler.add_command(String::from("pick"), cmds::cmd_misc::pick);
    handler.add_command(String::from("range"), cmds::cmd_misc::range);
    handler.add_command(String::from("hentai"), cmds::cmd_misc::is_hentai);
    handler.add_command(String::from("cfb"), cmds::cmd_misc::cfb);
    handler.add_command(String::from("help"), cmds::cmd_misc::help);
    handler.add_command(String::from("cmds"), cmds::cmd_misc::cmds);
    handler.add_command(String::from("poll"), cmds::cmd_misc::poll);
    handler.add_command(String::from("repo"), cmds::cmd_misc::repo);
    handler.add_command(String::from("weekly"), cmds::cmd_misc::weekly);

    // ADMIN COMMANDS
    handler.add_command(String::from("set"), cmds::cmd_admin::set_command);

    //secret commands
    handler.add_command(String::from("strive"), cmds::cmd_misc::strive);
    handler.add_command(String::from("fsr"), cmds::cmd_misc::fsr);
    handler.add_command(String::from("iloveshadowhearts:fromthenewworld"), cmds::cmd_misc::shftnw);

    let config =
    ClientConfig::new_simple(StaticLoginCredentials::new(bot_username.clone(), Some(oauth_token)));
    let (mut incoming_messages, client) = Twitch_Client::new(config);

    let clone = client.clone();
    let join_handle = tokio::spawn(async move
    {
        while let Some(message) = incoming_messages.recv().await
        {
            // TODO: FIX MALFORMED TAG ERROR PROC
            if let Ok(ServerMessage::Privmsg(msg)) = ServerMessage::try_from(message)
            {
                let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
                const COLOR_FLAG: bool = true;
                match COLOR_FLAG
                {
                    true =>
                    {
                        //TODO: CHANGE THIS
                        //msg.name_color.map(|g| g.g).unwrap_or(0)
                        let r: &u8 = match &msg.name_color.as_ref()
                        {
                            Some(r) => &r.r,
                            None => &0,
                        };
                        let g: &u8 = match &msg.name_color.as_ref()
                        {
                            Some(g) => &g.g,
                            None => &0
                        };
                        let b: &u8 = match &msg.name_color.as_ref()
                        {
                            Some(b) => &b.b,
                            None => &0
                        };

                        println!("[{}] #{} <{}>: {}", dt_fmt.truecolor(138, 138, 138), msg.channel_login.truecolor(117, 97, 158), &msg.sender.name.truecolor(*r, *g, *b), msg.message_text)
                    },
                    false => println!("[{}] #{} <{}>: {}", dt_fmt, msg.channel_login, &msg.sender.name, msg.message_text),
                    _ => panic!(),
                }
                handle_priv(clone.clone(), bot_username.clone(), msg, &handler).await;
            }
        }
    });
    for channel in channels
    {
        client.join(channel.to_lowercase()).unwrap();
    }
    //client.join(channel).unwrap();
    join_handle.await.unwrap();
    Ok(())
}

// Handle Commands
async fn handle_priv(client: Twitch_Client, _bot_username: String, msg: PrivmsgMessage, handler: &EventHandler)
{
    //tracing::info!("Received message: {:#?}", msg);

    if let Some(_runtype) = commands::Runtype::try_from_msg(&msg.message_text)
    {
        let mut proc_msg: String = msg.message_text.to_lowercase().clone();
        proc_msg = String::from(&proc_msg[1..]); // send the name forced lowercase for case insensitivity /*name.len()*/
        let text = proc_msg.as_str();
        let (name_str, _args_start) = match text.split_once(' ')
        {
            Some((name_str, args_start)) => (name_str, args_start),
            None => (text, ""),
        };
        // TODO: parameterize ARGS
        handler.execute_command(String::from(name_str), client, msg).await.unwrap();
    }
}
