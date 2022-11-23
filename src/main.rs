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
pub mod helpers;
use crate::commands::Twitch_Client;
use crate::commands::Callback;
use crate::commands::EventHandler;

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
    //let mut cmd_map: HashMap<&str, Callback/*(String, String)Box<commands::Command>*/> = HashMap::new();
    //cmd_map.insert("dreamboumtweet", &commands::dreamboumtweet());
    //cmd_map.insert("speedgame", commands::speedgame());
    //Box::new(||{commands::dreamboumtweet()}));
    let mut handler = EventHandler { command_map: HashMap::new() };
    handler.add_command("dreamboumtweet".to_string(), commands::dreamboumtweet);
    //let map: HashMap<&str, Callback> = handler.command_map.clone();
    if msg.message_text.to_lowercase().starts_with("!")
       || msg.message_text.to_lowercase().starts_with("?")
    {
        let mut name: String = msg.message_text.clone();
        name = String::from(&name[1..name.len()]);
        //println!("{} {}", msg.message_text, name);
        tokio::spawn(commands::execute_command(name, client, msg, handler.command_map));
    }
}