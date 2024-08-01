use chrono::NaiveDateTime;
use twitch_irc::message::PrivmsgMessage;

use crate::db_ops::*;
use crate::models::*;

pub async fn me(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String> {
    let user_data: BACUser = query_user_data(&msg_ctx.sender.name);
    match runtype {
        b'!' => {
            let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
            let days = ndt_now
                .signed_duration_since(user_data.date_added)
                .num_days();
            //return Ok(format!("| Nick: {} | Commands: {} | Date Added: {} |", msg_ctx.sender.name, user_data.num_commands, user_data.date_added));
            match days {
                0 => {
                    return Ok(format!(
                        "{} became a BAC user today! They have used {} commands.",
                        msg_ctx.sender.name, user_data.num_commands
                    ))
                }
                1 => {
                    return Ok(format!(
                        "{} has been a BAC user for {} day. They have used {} commands.",
                        msg_ctx.sender.name, days, user_data.num_commands
                    ))
                }
                _ => {
                    return Ok(format!(
                        "{} has been a BAC user for {} days. They have used {} commands.",
                        msg_ctx.sender.name, days, user_data.num_commands
                    ))
                }
            }
        }
        b'?' => {
            return Ok(
                "This command returns information based on your usage of the bot.".to_string(),
            );
        }
        b'#' => {
            if user_data.num_commands == 1 {
                return Ok(format!(
                    "{} has used {} command!",
                    msg_ctx.sender.name, user_data.num_commands
                ));
            } else {
                return Ok(format!(
                    "{} has used commands {} times!",
                    msg_ctx.sender.name, user_data.num_commands
                ));
            }
        }
        _ => Ok(String::from("")),
    }
}
