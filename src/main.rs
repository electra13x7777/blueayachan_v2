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
    time::Duration,
    collections::HashMap,
};
use chrono::
{
    NaiveDateTime,
};
use twitch_irc::
{
    login::StaticLoginCredentials,
    message::{PrivmsgMessage, ServerMessage},
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};
pub mod commands;
use crate::commands::
{
    Twitch_Client,
    Callback,
    EventHandler,
    Runtype,
};

pub mod helpers;
use crate::helpers::readlines_to_vec;
use crate::helpers::readlines_to_map;
pub mod db_connect;
pub mod models;
pub mod schema;
pub mod db_ops;
//pub mod test_db_stuff;
#[macro_use]
extern crate diesel;
use crate::db_ops::*;
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
    let channel = env::var("CHANNEL_NAME").context("missing CHANNEL_NAME environment variable")?;
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
    handler.add_command(String::from("test"), commands::test_command);
    handler.add_command(String::from("dreamboumtweet"), commands::dreamboumtweet);
    handler.add_command(String::from("demongacha"), commands::demongacha);
    handler.add_command(String::from("savedemon"), commands::savedemon);
    handler.add_command(String::from("hornedanimegacha"), commands::hornedanimegacha);
    handler.add_command(String::from("melty"), commands::melty);
    handler.add_command(String::from("lumina"), commands::lumina);
    handler.add_command(String::from("melee"), commands::melee);
    handler.add_command(String::from("soku"), commands::soku);
    handler.add_command(String::from("bbcf"), commands::bbcf);
    handler.add_command(String::from("ggxxacplusr"), commands::ggxxacplusr);
    handler.add_command(String::from("akb"), commands::akb);
    handler.add_command(String::from("vsav"), commands::vsav);

    handler.add_command(String::from("me"), commands::me);
    handler.add_command(String::from("args"), commands::test_args);
    handler.add_command(String::from("help"), commands::help);
    handler.add_command(String::from("cmds"), commands::cmds);
    handler.add_command(String::from("poll"), commands::poll);
    //handler.add_command(String::from("speedgame"), commands::speedgame);

    let config =
    ClientConfig::new_simple(StaticLoginCredentials::new(bot_username.clone(), Some(oauth_token)));
    let (mut incoming_messages, client) = Twitch_Client::new(config);

    let clone = client.clone();
    let join_handle = tokio::spawn(async move
    {
        while let Some(message) = incoming_messages.recv().await
        {
            if let ServerMessage::Privmsg(msg) = message
            {
                let dt_fmt = chrono::offset::Local::now().format("%H:%M:%S").to_string();
                println!("[{}] #{} <{}>: {}", dt_fmt, msg.channel_login, &msg.sender.name, msg.message_text);
                handle_priv(clone.clone(), bot_username.clone(), msg, &handler).await;
            }
        }
    });

    client.join(channel).unwrap();

    join_handle.await.unwrap();
    Ok(())
}

// Handle Commands
async fn handle_priv(client: Twitch_Client, bot_username: String, msg: PrivmsgMessage, handler: &EventHandler)
{
    //tracing::info!("Received message: {:#?}", msg);

    if let Some(runtype) = commands::Runtype::try_from_msg(&msg.message_text)
    {
        let mut proc_msg: String = msg.message_text.to_lowercase().clone();
        proc_msg = String::from(&proc_msg[1..]); // send the name forced lowercase for case insensitivity /*name.len()*/
        let text = proc_msg.as_str();
        let (name_str, args_start) = match text.split_once(' ')
        {
            Some((name_str, args_start)) => (name_str, args_start),
            None => (text, ""),
        };
        // TODO: parameterize ARGS
        handler.execute_command(String::from(name_str), client, msg).await.unwrap();
    }
}
