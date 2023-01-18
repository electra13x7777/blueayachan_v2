


use chrono::NaiveDateTime;

use crate::{commands::{Command, Runtype}};

use crate::db_ops::*;
use crate::models::*;

pub async fn me(command: Command) -> anyhow::Result<String>
{
    let user_data: BACUser = query_user_data(&command.msg.sender.name);
    match command.runtype
    {
        Runtype::Command =>
        {
            let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
            let days = ndt_now.signed_duration_since(user_data.date_added).num_days();
            //return Ok(format!("| Nick: {} | Commands: {} | Date Added: {} |", msg_ctx.sender.name, user_data.num_commands, user_data.date_added));
            match days
            {
                0 => return Ok(format!("{} became a BAC user today! They have used {} commands.", command.msg.sender.name, user_data.num_commands)),
                1 => return Ok(format!("{} has been a BAC user for {} day. They have used {} commands.", command.msg.sender.name, days, user_data.num_commands)),
                _ => return Ok(format!("{} has been a BAC user for {} days. They have used {} commands.", command.msg.sender.name, days, user_data.num_commands))
            }
        },
        Runtype::Help =>
        {
            return Ok("This command returns information based on your usage of the bot.".to_string());
        },
        Runtype::Hash =>
        {
            if user_data.num_commands == 1
            {
                return Ok(format!("{} has used {} command!", command.msg.sender.name, user_data.num_commands));
            }
            else
            {
                return Ok(format!("{} has used commands {} times!", command.msg.sender.name, user_data.num_commands));
            }
        },
        _ => Ok(String::from("")),
    }
}