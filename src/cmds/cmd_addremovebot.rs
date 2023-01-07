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

// BLUEAYACHAN CHANNEL ONLY COMMANDS
pub async fn add_bot(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    return Ok(String::from(""));
}
pub async fn remove_bot(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    return Ok(String::from(""));
}
