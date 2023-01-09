
use std::
{
    io::{prelude::*},
};


use twitch_irc::
{
    message::{PrivmsgMessage},
};


use crate::db_ops::*;
use crate::models::*;

// BROADCASTER ONLY COMMANDS

// EXAMPLE: !set <COMMAND_NAME> <ARG>
//          ARGS
//			0 : ARG1 = COMMAND_NAME
//			1 : ARG2 = COMMAND_OP
//			2 : ARG3 = TIMEOUT_DUR (only for timeout)
//           x    0   1         2           ALIAS
//          !set cmd on
//          !set cmd off
//          !set cmd toggle
//          !set cmd timeout <SECONDS>       t
//          !set cmd broadcaster             me, b
//          !set cmd mod                     m
//          !set cmd all                     a

pub async fn set_command(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    const ADMIN_CMDS: &'static [&'static str] = &["on", "off", "toggle", "timeout", "broadcaster", "mod", "all"];
    const ADMIN_CMDS_ALIAS: &'static [&'static str] = &["t", "me", "b", "m", "a"];
    let badges: Vec<String> = msg_ctx.badges.iter().map(|b| b.name.clone()).collect();
    if !badges.contains(&"broadcaster".to_string()) && &msg_ctx.sender.name.to_lowercase() != "electra_rta"
    {
        return Ok(format!("{}, this is a broadcaster only command!", msg_ctx.sender.name));
    }
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
            let argv_s: Vec<String> = args.split(' ').map(|s| s.to_string()).collect();
            // validate
            // check input 1
            let cmds: Vec<BACommand> = query_cmd_to_vec();
            let mut is_valid_cmd: bool = false;
            let mut id_val: i32 = 0;
            // CHECK COMMAND NAME
            for c in cmds
            {
                if &c.name == &argv_s[0]
                {
                    is_valid_cmd = true;
                    id_val = c.id;
                }
            }
            if !is_valid_cmd{return Ok(String::from("Invalid Command. ARG1"));}
            // CHECK COMMAND OP
            if !ADMIN_CMDS.contains(&argv_s[1].as_str()) && !ADMIN_CMDS_ALIAS.contains(&argv_s[1].as_str())
            {
                return Ok(String::from("Invalid Command Op. ARG2"));
            }
            // maybe check arg count?
            let bacchannel: BACUser = query_user_data(msg_ctx.channel_login.clone());
            match argv_s[1].as_str()
            {
                "on" =>
                {
                    let res: (bool, String) = set_channel_command_active(bacchannel, id_val);
                    match res.0
                    {
                        true => {return Ok(res.1);},
                        false => {return Ok(res.1);},
                    }
                },
                "off" =>
                {
                    let res: (bool, String) = set_channel_command_inactive(bacchannel, id_val);
                    match res.0
                    {
                        true => {return Ok(res.1);},
                        false => {return Ok(res.1);},
                    }
                },
                "toggle" =>
                {
                    let res: (bool, String) = toggle_channel_command_active(bacchannel, id_val);
                    match res.0
                    {
                        true => {return Ok(res.1);},
                        false => {return Ok(res.1);},
                    }
                }
                // TIMEOUT REQUIRES A 3RD POSITIONAL ARGUMENT
                "timeout" | "t" =>
                {
                    // CHECK NUMERIC
                    for c in argv_s[2].chars()
                    {
                        if !c.is_numeric()
                        {
                            return Ok(String::from("Bad input for ARG3. Please make sure that your last argument for augmenting the command timeout is a number between 0 and 3600 (upper bound inclusive)"));
                        }
                    }
                    let timeout_val: i32 = argv_s[2].parse::<i32>().unwrap();
                    // CHECK VALID TIMEOUT VALUE
                    if timeout_val > 3600
                    {
                        return Ok(String::from("Bad input for ARG3. Please make sure that your last argument for augmenting the command timeout is a number between 0 and 3600 (upper bound inclusive)"));
                    }
                    // IF 0 IS PROVIDED WE SET THE TIMEOUT TO INACTIVE
                    if timeout_val == 0
                    {
                        //let res_dur: (bool, String) = set_channel_command_timeout_duration(timeout_val);
                        let res: (bool, String) = set_channel_command_timeout_off(bacchannel, id_val);
                        match res.0
                        {
                            true => {return Ok(res.1);},
                            false => {return Ok(res.1);},
                        }
                    }
                    let to_res: (bool, String) = set_channel_command_timeout_on(bacchannel, id_val);
                    match to_res.0
                    {
                        true =>
                        {
                            let dur_res: (bool, String) = set_channel_command_timeout_duration(query_user_data(msg_ctx.channel_login.clone()), id_val, timeout_val);
                            match dur_res.0
                            {
                                true => {return Ok(dur_res.1);},
                                false => {return Ok(dur_res.1);},
                            }
                        },
                        false =>
                        {
                            let dur_res: (bool, String) = set_channel_command_timeout_duration(query_user_data(msg_ctx.channel_login.clone()), id_val, timeout_val);
                            match dur_res.0
                            {
                                true => {return Ok(dur_res.1);},
                                false => {return Ok(dur_res.1);},
                            }
                        },
                    }
                },
                "broadcaster" | "b" =>
                {
                    let res: (bool, String) = set_channel_command_broadcaster_only(bacchannel, id_val);
                    match res.0
                    {
                        true => {return Ok(res.1);},
                        false => {return Ok(res.1);},
                    }
                },
                "mod" | "m" =>
                {
                    let res: (bool, String) = set_channel_command_mod_only(bacchannel, id_val);
                    match res.0
                    {
                        true => {return Ok(res.1);},
                        false => {return Ok(res.1);},
                    }
                },
                "all" | "a" =>
                {
                    let res: (bool, String) = set_channel_command_all(bacchannel, id_val);
                    match res.0
                    {
                        true => {return Ok(res.1);},
                        false => {return Ok(res.1);},
                    }
                },
                _ =>
                {
                    return Ok("Invalid command op used!".to_string());
                },
            }
        },
        b'?' =>
        {
            return Ok("This command sets the privilages and timeouts of a given command for a channel. It can only be used by the channel owner. Please refer to this pastebin for a full list of supported use cases: https://pastebin.com/z6zxSiB5".to_string());
        },
        b'#' =>
        {
            Ok(String::from(""))
        },
        _ => Ok(String::from("")),
    }
}

// TODO: REMOVE FUNCTIONS BELOW

// TOGGLE COMMAND - turns a command on or off
pub async fn toggle_command(_runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    let badges: Vec<String> = msg_ctx.badges.iter().map(|b| b.name.clone()).collect();
    if !badges.contains(&"broadcaster".to_string())
    {
        return Ok(format!("{}, this is a broadcaster only command!", msg_ctx.sender.name));
    }
    return Ok(String::from(""));
}

// TOGGLE COMMAND TIMEOUT - turns the timeout of a command on or off
pub async fn toggle_command_timeout(_runtype: u8, _msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    return Ok(String::from(""));
}

//     CHANGE COMMAND TIMEOUT <INT> - changes the timeout value in seconds of a command in the channel
//                                  - if the user inputs 0 then it turns off the timeout by default
pub async fn change_command_timeout(_runtype: u8, _msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    return Ok(String::from(""));
}

