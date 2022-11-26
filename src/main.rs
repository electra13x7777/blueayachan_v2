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
use chrono;
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
pub mod db_connect;
pub mod models;
pub mod schema;
pub mod db_ops;
#[macro_use]
extern crate diesel;
use crate::db_ops::{insert_dbtweet, query_dbtweet_to_vec};

//type Client = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;

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

    let config =
    ClientConfig::new_simple(StaticLoginCredentials::new(bot_username, Some(oauth_token)));
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
                handle_priv(clone.clone(), msg).await;
            }
        }
    });

    client.join(channel).unwrap();

    join_handle.await.unwrap();
    Ok(())
}

// Handle Commands
async fn handle_priv(client: Twitch_Client, msg: PrivmsgMessage)
{
    //tracing::info!("Received message: {:#?}", msg);
    let runtypes: String = String::from("!?#~`");
    let mut handler = EventHandler { command_map: HashMap::new() };
    handler.add_command(String::from("test"), commands::test_command);
    handler.add_command(String::from("dreamboumtweet"), commands::dreamboumtweet);

    if let Some(runtype) = commands::Runtype::try_from_msg(&msg.message_text)
    {
        let mut name: String = msg.message_text.to_lowercase().clone();
        name = String::from(&name[1..]); // send the name forced lowercase for case insensitivity /*name.len()*/
        tokio::spawn(commands::execute_command(name, client, msg, handler.command_map));
    }
}