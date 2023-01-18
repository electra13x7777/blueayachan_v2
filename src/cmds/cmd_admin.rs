

use twitch_irc::
{
    message::{PrivmsgMessage},
};


use crate::{db_ops::*, commands::{Runtype, Command}};
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

pub async fn set_command(command: Command) -> anyhow::Result<String>
{
    if !command.msg.badges.iter().any(|badge| badge.name == "broadcaster") && command.msg.sender.name.to_lowercase() != "electra_rta"
    {
        return Ok(format!("{}, this is a broadcaster only command!", command.msg.sender.name));
    }
    match command.runtype
    {
        Runtype::Command =>
        {
            let argv_s: Vec<&str> = command.args().split(' ').collect();
            // validate
            // check input 1
            let cmds: Vec<BACommand> = query_cmd_to_vec();
            let Some(id_val) = cmds.iter().find_map(|c| (c.name == argv_s[0]).then_some(c.id))
            else
            {
                return Ok(String::from("Invalid Command. ARG1"));
            };
            // maybe check arg count?
            let bacchannel: BACUser = query_user_data(&command.msg.channel_login);
            let res: &str = match argv_s[1]
            {
                "on" => set_channel_command_active(&bacchannel, id_val),
                "off" => set_channel_command_inactive(&bacchannel, id_val),
                "toggle" => toggle_channel_command_active(&bacchannel, id_val),
                // TIMEOUT REQUIRES A 3RD POSITIONAL ARGUMENT
                "timeout" | "t" =>
                {
                    let timeout_val = match argv_s[2].parse::<i32>()
                    {
                        Ok(timeout_val) if (0..=3600).contains(&timeout_val) =>
                        {
                            timeout_val
                        }
                        _ => return Ok(String::from("Bad input for ARG3. Please make sure that your last argument for augmenting the command timeout is a number between 0 and 3600 (upper bound inclusive)")),
                    };
                    // IF 0 IS PROVIDED WE SET THE TIMEOUT TO INACTIVE
                    if timeout_val == 0
                    {
                        //let res_dur: (bool, String) = set_channel_command_timeout_duration(timeout_val);
                        let res: &str = set_channel_command_timeout_off(&bacchannel, id_val);
                        return Ok(res.to_string());
                    }
                    let _ = set_channel_command_timeout_on(&bacchannel, id_val);
                    let dur_res: &str = set_channel_command_timeout_duration(&query_user_data(&command.msg.channel_login), id_val, timeout_val);
                    dur_res
                },
                "broadcaster" | "b" => set_channel_command_broadcaster_only(&bacchannel, id_val),
                "mod" | "m" => set_channel_command_mod_only(&bacchannel, id_val),
                "all" | "a" => set_channel_command_all(&bacchannel, id_val),
                _ =>
                {
                    return Ok(String::from("Invalid Command Op. ARG2"));
                },
            };
            return Ok(res.to_string())
        },
        Runtype::Help =>
        {
            return Ok("This command sets the privilages and timeouts of a given command for a channel. It can only be used by the channel owner. Please refer to this pastebin for a full list of supported use cases: https://pastebin.com/z6zxSiB5".to_string());
        },
        _ => Ok(String::from("")),
    }
}

// TODO: REMOVE FUNCTIONS BELOW

// TOGGLE COMMAND - turns a command on or off
pub async fn toggle_command(_runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    if !msg_ctx.badges.iter().any(|badge| badge.name == "broadcaster")
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

