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

use crate::helpers::readlines_to_vec;
use crate::db_ops::*;
use crate::models::*;

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