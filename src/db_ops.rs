use crate::models::*;

use crate::db_connect::
{
    establish_connection,
    //establish_connection_async
};
use diesel::
{
    prelude::*,
    select
};
use diesel::pg::PgConnection;
/*use diesel_async::
{
    AsyncConnection,
    AsyncPgConnection,
    RunQueryDsl
};*/
use diesel::dsl::
{
    //now,
    exists
};
use chrono::
{
    NaiveDateTime
};


///////////////////////////////////////////////////////////////////////////////
//                           USER, ROLES, USERROLES                          //
///////////////////////////////////////////////////////////////////////////////

// called when a new user sends a valid request to execute a commands
pub fn handle_bac_user_in_db(user_nick_str: String, twitch_id_str: String)
{
    use crate::schema::blueayachanuser::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let user_nick_lower: String = user_nick_str.to_lowercase();
    // CHECK
    let user_exists: bool = select(exists(blueayachanuser.filter(user_nick.eq(&user_nick_lower))))
        .get_result(&mut connection).unwrap();

    if !user_exists // FIRST TIME USING A COMMAND
    {
        let first_command: i32 = 1;
        let nt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();//.format("%H:%M:%S");
        let new_bac_user = NewBACUser
        {user_nick: &user_nick_lower, num_commands: &first_command, date_added: &nt_now, twitch_id: &twitch_id_str};
        // TODO: ADD A CHECK HERE QUERYING BY TWITCH_ID TO SEE IF THAT USER HAS EXISTED PREVIOUSLY IF TRUE MIGRATE THAT USERDATA TO NEW USER_NICK
        // insert
        diesel::insert_into(blueayachanuser)
            .values(&new_bac_user)
            .execute(&mut connection)
            .expect("Error inserting new user");
    }
    else // ALREADY IN DATABASE
    {
        let bacuser: BACUser = query_user_data(user_nick_str);
        if bacuser.twitch_id == "Unregistered"
        {
            // TODO: ADD CHECK HERE TO SEE IF THE ID IS ALREADY IN DB TABLE
            diesel::update(blueayachanuser
                .filter(user_nick.eq(&user_nick_lower)))
                .set(twitch_id.eq(twitch_id_str))
                .execute(&mut connection);
        }
        let _updated_row = diesel::update(blueayachanuser.filter(user_nick.eq(user_nick_lower)))
            .set(num_commands.eq(num_commands+1))
            .execute(&mut connection);
    }
}
//pub fn handle_id(){}

pub fn query_user_data(user_nick_str: String) -> BACUser
{
    use crate::schema::blueayachanuser::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let user_nick_lower: String = user_nick_str.to_lowercase();
    let result = blueayachanuser.filter(user_nick.eq(&user_nick_lower)).first::<BACUser>(&mut connection).expect("Oh no!");
    return result;
}

// ONLY FOR COMMAND IMPLEMENTATION USE
pub fn query_user_data_by_tid(twitch_id_str: String) -> BACUser
{
    use crate::schema::blueayachanuser::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let result = blueayachanuser.filter(twitch_id.eq(&twitch_id_str)).first::<BACUser>(&mut connection).expect("Oh no!");
    return result;
}

/*
pub fn query_total_commands() -> Option<i64>
{
    use crate::schema::blueayachanuser::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let result: Option<i64> = Some(blueayachanuser.select(sum(num_commands)).first(&mut connection).unwrap());
    return result;

}*/

// BACKEND ONLY!! WILL NEVER EXECUTE IN OUR EVENT LOOP

pub fn insert_role(role_str: String)
{
    use crate::schema::roles::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let nt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
    let new_role = NewRole{role_name: &role_str, date_added: &nt_now};
    diesel::insert_into(roles)
    .values(&new_role)
    .execute(&mut connection)
    .expect("Error inserting new role");
}
/* // TODO: BAC USER ROLES
pub fn insert_bac_user_role(user_nick_str: String, role_str: String)
{
    use crate::schema::blueayachanuser::dsl::*;
    use crate::schema::roles::dsl::*;
    use crate::schema::blueayachanuser_roles::dsl::*;
    let mut connection: PgConnection = establish_connection();

    let user = blueayachanuser.find(user_nick_str).first::<User>(&mut connection).expect("Error loading user");
    let post_list = Post::belonging_to(&user)
    .load::<Post>(&connection)
    .expect("Error loading posts");

    let nt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
    let new_userrole = NewBAC_User_Role{role_name: &role_str, date_added: &nt_now};
    diesel::insert_into(roles)
    .values(&new_role)
    .execute(&mut connection)
    .expect("Error inserting new role");
}
*/
// called when existing user sends a request to execute a command
/*



pub fn update_role(){}
pub fn insert_bac_user_role(){}
pub fn update_bac_user_role(){}
*/

///////////////////////////////////////////////////////////////////////////////
//                              DREAMBOUMTWEETS                              //
///////////////////////////////////////////////////////////////////////////////

pub fn insert_dbtweet(tweet_str: String)
{
    // parse out members
    let splitpoint: usize = 13;
    let length = tweet_str.len();
    let tweet_ctx: &str = &tweet_str[0..length-splitpoint];
    let date_str: &str = &tweet_str[length-splitpoint..];

    use crate::schema::dreamboumtweets::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let new_dbtweet = New_DBTweet{tweet: tweet_ctx, tweet_date: date_str};
    // insert
    diesel::insert_into(dreamboumtweets)
    .values(&new_dbtweet)
    .execute(&mut connection)
    .expect("Error inserting tweet");
}

// TODO: QUERY BY INDEX
pub fn query_dbtweet_to_vec() -> Vec<(String, String)>
{
    use crate::schema::dreamboumtweets::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let results = dreamboumtweets
    .load::<DBTweet>(&mut connection)
    .expect("Error querying tweets");
    let mut out: Vec<(String, String)> = Vec::new();
    for dbtweet in results
    {
        let vals: (String, String) = (dbtweet.tweet, dbtweet.tweet_date);
        out.push(vals);
    }
    return out;
}

pub fn query_single_dbtweet(q_id: i32) -> String
{
    // do a check here first
    assert!(q_id <= 6569);
    use crate::schema::dreamboumtweets::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let result = dreamboumtweets.find(q_id).first::<DBTweet>(&mut connection).unwrap();
    return result.tweet;
}


/*
pub async fn query_dbtweet_async(q_id: i32) -> String
{
assert!(q_id <= 6569);
use crate::schema::dreamboumtweets::dsl::*;
let mut connection: AsyncPgConnection = establish_connection_async();
let result = dreamboumtweets.find(q_id).first::<DBTweet>(&mut connection).unwrap();
return result.tweet;
}*/



///////////////////////////////////////////////////////////////////////////////
//                           GACHA COMMANDS RELATED                          //
///////////////////////////////////////////////////////////////////////////////

// DEMONGACHA //

pub fn insert_demon(name_str: String, link_str: String)
{
    use crate::schema::nocturnedemons::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let new_demon = New_NDemon{demon_name: &name_str[..], demon_img_link: &link_str[..]};
    // insert
    diesel::insert_into(nocturnedemons)
    .values(&new_demon)
    .execute(&mut connection)
    .expect("Error inserting demon");
}

pub fn query_demon(q_id: i32) -> NDemon
{
    // do a check here first
    assert!(q_id <= 184);
    use crate::schema::nocturnedemons::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let result = nocturnedemons.find(q_id).first::<NDemon>(&mut connection).unwrap();
    return result;
}

pub fn handle_user_last_demon(bacuser: BACUser, demon: &NDemon, rarity: &i32)
{
    // USER WILL ALWAYS BE IN BACUSER
    // PARAMETERS MUST NEVER BE STALE
    use crate::schema::bac_user_demons::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let user_exists: bool = select(exists(bac_user_demons.filter(user_id.eq(&bacuser.id))))
    .get_result(&mut connection).unwrap();
    if !user_exists
    {
        // set user demon defaults
        let sd_id = 62;
        let sd_r = 1;
        let ld_id = &demon.id;
        let ld_r = rarity;

        let new_bac_user_demon = New_SavedNDemon
        {
            user_id: &bacuser.id, saved_demon_id: &sd_id, saved_demon_rarity: &sd_r,
            last_demon_id: ld_id, last_demon_rarity: ld_r
        };
        // insert
        diesel::insert_into(bac_user_demons)
        .values(&new_bac_user_demon)
        .execute(&mut connection)
        .expect("Error inserting new user");
    }
    else
    {
        // When user exists only update the last demon fields
        diesel::update(bac_user_demons.filter(user_id.eq(&bacuser.id)))
        .set(last_demon_id.eq(&demon.id))
        .execute(&mut connection).expect("Error updating last demon ID");
        diesel::update(bac_user_demons.filter(user_id.eq(&bacuser.id)))
        .set(last_demon_rarity.eq(rarity))
        .execute(&mut connection).expect("Error updating last demon RARITY");
    }
}

//helper
pub fn query_user_demon(bacuser: &BACUser) -> Option<SavedNDemon>
{
    use crate::schema::bac_user_demons::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let user_exists: bool = select(exists(bac_user_demons.filter(user_id.eq(&bacuser.id))))
    .get_result(&mut connection).unwrap();
    //println!("UID: {}", &user_exists);
    if !user_exists
    {
        // WE WILL DO NOTHING
        
        return None;
    }
    else
    {
        let result = bac_user_demons.filter(user_id.eq(&bacuser.id)).first::<SavedNDemon>(&mut connection).expect("Error finding user");
        return Some(result);
    }
}

pub fn save_user_demon(bacuser: BACUser)
{
    use crate::schema::bac_user_demons::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let user_exists: bool = select(exists(bac_user_demons.filter(user_id.eq(&bacuser.id))))
    .get_result(&mut connection).unwrap();
    if !user_exists
    {
        // WE WILL DO NOTHING
        return;
    }
    else
    {
        // When user exists only update the last demon fields
        let sud: SavedNDemon = query_user_demon(&bacuser).expect("Error Querying User Demon Data");

        diesel::update(bac_user_demons.filter(user_id.eq(&bacuser.id)))
                .set(saved_demon_id.eq(&sud.last_demon_id))
        .execute(&mut connection).expect("Error updating saved demon ID");
        diesel::update(bac_user_demons.filter(user_id.eq(&bacuser.id)))
                .set(saved_demon_rarity.eq(&sud.last_demon_rarity))
                .execute(&mut connection).expect("Error updating saved demon RARITY");
    }
}

pub fn query_pic_timeout(bacuser: &BACUser) -> Option<PicTimeout>
{
    use crate::schema::pictimeout::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let user_exists: bool = select(exists(pictimeout.filter(user_id.eq(&bacuser.id))))
    .get_result(&mut connection).unwrap();

    if !user_exists
    {
        // WE WILL DO NOTHING

        return None;
    }
    else
    {
        let result = pictimeout.filter(user_id.eq(&bacuser.id)).first::<PicTimeout>(&mut connection).expect("Error finding user");
        return Some(result);
    }
}

pub fn handle_pic_timeout(bacuser: BACUser, ndt_now: NaiveDateTime, timeout: i64) -> (bool, i64)
{
    use crate::schema::pictimeout::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let user_exists: bool = select(exists(pictimeout.filter(user_id.eq(&bacuser.id))))
    .get_result(&mut connection).unwrap();
    if !user_exists
    {
        // set user timeout defaults
        let npt = NewPicTimeout
        {
            user_id: &bacuser.id, last_pic: &ndt_now
        };
        // insert
        diesel::insert_into(pictimeout)
            .values(&npt)
            .execute(&mut connection)
            .expect("Error inserting new user pic timeout");
        return (true, 0);
    }
    else
    {
        
        let pt = match query_pic_timeout(&bacuser)
        {
            Some(pt) => pt,
            None => panic!() // will never happen
        };
        let diff: i64 = ndt_now.signed_duration_since(pt.last_pic).num_seconds();
        if diff >= timeout
        {
            diesel::update(pictimeout.filter(user_id.eq(&bacuser.id)))
                .set(last_pic.eq(&ndt_now))
                .execute(&mut connection).expect("Error updating last pic timestamp");
            return (true, 0);
        }
        return (false, diff);
    }
}

////////////////////////////////////////////////////////////
//                  BOT CHANNEL

// ADD BOT TO CHANNEL
pub fn insert_botchannel(user_nick_str: String) -> bool
{
    use crate::schema::botchannels::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let channel_exists: bool = select(exists(botchannels.filter(channel_name.eq(&user_nick_str)))).get_result(&mut connection).unwrap();
    if !channel_exists
    {
        let bacuser: BACUser = query_user_data(user_nick_str);
        let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
        let new_channel = NewBotChannel{channel_name: &bacuser.user_nick, channel_twitch_id: &bacuser.twitch_id, last_updated: &ndt_now};
        diesel::insert_into(botchannels)
            .values(&new_channel)
            .execute(&mut connection)
            .expect("Error inserting new user");
        return true;
    }
    return false;
}

// REMOVE BOT FROM CHANNEL

//////////////////////////////////////////////////////////////
//                  COMMAND ADMIN

pub fn query_command_id(command_name_str: &str) -> Option<i32>
{
    use crate::schema::blueayacommands::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let command_exists: bool = select(exists(blueayacommands.filter(name.eq(command_name_str)))).get_result(&mut connection).unwrap();
    if !command_exists
    {
        return None;
    }
    else
    {
        let result = blueayacommands.filter(name.eq(&command_name_str)).first::<BACommand>(&mut connection).expect("Oh no!");
        return Some(result.id);
    }
}

// TODO: ADD DEFAULT INSERT FOR CHANNEL COMMANDS: SEE PICTIMEOUT
pub fn insert_channel_command(bacchannel: BACUser, command_id_val: i32)
{
    use crate::schema::channelcommands::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let nt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
    let new_cc = NewChannelCommands
    {
        channel_bac_id: &bacchannel.id, command_id: &command_id_val,
        is_active: &true, is_broadcaster_only: &false, is_mod_only: &false,
        has_timeout: &false, timeout_dur: &30,/*num_used: &0,*/ last_updated: &nt_now
    };
    diesel::insert_into(channelcommands)
            .values(&new_cc)
            .execute(&mut connection)
            .expect("Error inserting new user");
}
// CHANNEL COMMANDS
// CONSTRAINT channelcommands_pkey PRIMARY KEY (channel_bac_id, command_id),
//
pub fn query_channel_command(bacchannel: BACUser, command_id_val: i32) -> Option<ChannelCommands>
{
    use crate::schema::channelcommands::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let channel_command_exists: bool = select(exists(channelcommands.filter(channel_bac_id.eq(&bacchannel.id).and(command_id.eq(&command_id_val)))))
    .get_result(&mut connection).unwrap();
    if !channel_command_exists
    {
        // NEEDS TO INSERT A COMMAND WITH DEFAULT FIELDS
        return None;
    }
    else
    {
        let result = channelcommands.filter(channel_bac_id.eq(&bacchannel.id).and(command_id.eq(&command_id_val))).first::<ChannelCommands>(&mut connection).expect("Error finding Channel Command");
        return Some(result);
    }
}

// SET CHANNEL COMMANDS
// MUST BE BROADCASTER
// ALL SUCCESSFUL RUNS WILL RETURN TRUE, FAILURE RETURNS FALSE

pub fn set_channel_command_active(bacchannel: BACUser, command_id_val: i32) -> (bool, String)
{
    use crate::schema::channelcommands::dsl::*;
    let ch_id = &bacchannel.id.clone();
    let cmd_id = &command_id_val.clone();
    let mut connection: PgConnection = establish_connection();
    let cc = match query_channel_command(bacchannel, command_id_val)
    {
        Some(cc) => cc,
        None => return (false, "Command not found".to_string())
    };
    let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
    if cc.is_active
    {
        return (false, "Command is already active".to_string());
    }
    diesel::update(channelcommands.filter(channel_bac_id.eq(&ch_id).and(command_id.eq(&cmd_id))))
            .set((is_active.eq(true), last_updated.eq(&ndt_now),))
            .execute(&mut connection).expect("Error updating channel command");
    return (true, "set to active".to_string());
}

pub fn set_channel_command_inactive(bacchannel: BACUser, command_id_val: i32) -> (bool, String)
{
    use crate::schema::channelcommands::dsl::*;
    let ch_id = &bacchannel.id.clone();
    let cmd_id = &command_id_val.clone();
    let mut connection: PgConnection = establish_connection();
    let cc = match query_channel_command(bacchannel, command_id_val)
    {
        Some(cc) => cc,
        None => return (false, "Command not found".to_string())
    };
    let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
    if !cc.is_active
    {
        return (false, "Command is already inactive".to_string());
    }
    diesel::update(channelcommands.filter(channel_bac_id.eq(&ch_id).and(command_id.eq(&cmd_id))))
            .set((is_active.eq(false), last_updated.eq(&ndt_now),))
            .execute(&mut connection).expect("Error updating channel command");
    return (true, "set to inactive".to_string());
}

//THIS IS A TOGGLE
pub fn toggle_channel_command_active(bacchannel: BACUser, command_id_val: i32) -> (bool, String)
{
    use crate::schema::channelcommands::dsl::*;
    let ch_id = &bacchannel.id.clone();
    let cmd_id = &command_id_val.clone();
    let mut connection: PgConnection = establish_connection();
    let cc = match query_channel_command(bacchannel, command_id_val)
    {
        Some(cc) => cc,
        None => return (false, "Command not found".to_string())
    };
    let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
    if cc.is_active
    {
        diesel::update(channelcommands.filter(channel_bac_id.eq(&ch_id).and(command_id.eq(&cmd_id))))
            .set((is_active.eq(false), last_updated.eq(&ndt_now),))
            .execute(&mut connection).expect("Error updating channel command");
        return (true, "set to inactive".to_string())
    }
    diesel::update(channelcommands.filter(channel_bac_id.eq(&ch_id).and(command_id.eq(&cmd_id))))
            .set((is_active.eq(true), last_updated.eq(&ndt_now),))
            .execute(&mut connection).expect("Error updating channel command");
    return (true, "set to active".to_string());
}

pub fn set_channel_command_broadcaster_only(bacchannel: BACUser, command_id_val: i32) -> (bool, String)
{
    use crate::schema::channelcommands::dsl::*;
    let ch_id = &bacchannel.id.clone();
    let cmd_id = &command_id_val.clone();
    let mut connection: PgConnection = establish_connection();
    let _cc = match query_channel_command(bacchannel, command_id_val)
    {
        Some(cc) => cc,
        None => return (false, "could not find command".to_string())
    };
    let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
    diesel::update(channelcommands.filter(channel_bac_id.eq(&ch_id).and(command_id.eq(&cmd_id))))
        .set((is_broadcaster_only.eq(true), is_mod_only.eq(false), last_updated.eq(&ndt_now),))
        .execute(&mut connection).expect("Error updating channel command");
    return (true, "set to broadcaster only".to_string());
}

pub fn set_channel_command_mod_only(bacchannel: BACUser, command_id_val: i32) -> (bool, String)
{
    use crate::schema::channelcommands::dsl::*;
    let ch_id = &bacchannel.id.clone();
    let cmd_id = &command_id_val.clone();
    let mut connection: PgConnection = establish_connection();
    let _cc = match query_channel_command(bacchannel, command_id_val)
    {
        Some(cc) => cc,
        None => return (false, "could not find command".to_string())
    };
    let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
    diesel::update(channelcommands.filter(channel_bac_id.eq(&ch_id).and(command_id.eq(&cmd_id))))
        .set((is_broadcaster_only.eq(false), is_mod_only.eq(true), last_updated.eq(&ndt_now),))
        .execute(&mut connection).expect("Error updating channel command");
    return (true, "set to broadcaster, mod, vip only".to_string());
}

pub fn set_channel_command_all(bacchannel: BACUser, command_id_val: i32) -> (bool, String)
{
    use crate::schema::channelcommands::dsl::*;
    let ch_id = &bacchannel.id.clone();
    let cmd_id = &command_id_val.clone();
    let mut connection: PgConnection = establish_connection();
    let _cc = match query_channel_command(bacchannel, command_id_val)
    {
        Some(cc) => cc,
        None => return (false, "could not find command".to_string())
    };
    let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
    diesel::update(channelcommands.filter(channel_bac_id.eq(&ch_id).and(command_id.eq(&cmd_id))))
        .set((is_broadcaster_only.eq(false), is_mod_only.eq(false), last_updated.eq(&ndt_now),))
        .execute(&mut connection).expect("Error updating channel command");
    return (true, "set to all users".to_string());
}

pub fn set_channel_command_timeout_on(bacchannel: BACUser, command_id_val: i32) -> (bool, String)
{
    use crate::schema::channelcommands::dsl::*;
    let ch_id = &bacchannel.id.clone();
    let cmd_id = &command_id_val.clone();
    let mut connection: PgConnection = establish_connection();
    let _cc = match query_channel_command(bacchannel, command_id_val)
    {
        Some(cc) => cc,
        None => return (false, "could not find command".to_string())
    };
    let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
    diesel::update(channelcommands.filter(channel_bac_id.eq(&ch_id).and(command_id.eq(&cmd_id))))
        .set((has_timeout.eq(true), last_updated.eq(&ndt_now),))
        .execute(&mut connection).expect("Error updating channel command");
    return (true, "timeout enabled".to_string());
}

pub fn set_channel_command_timeout_off(bacchannel: BACUser, command_id_val: i32) -> (bool, String)
{
    use crate::schema::channelcommands::dsl::*;
    let ch_id = &bacchannel.id.clone();
    let cmd_id = &command_id_val.clone();
    let mut connection: PgConnection = establish_connection();
    let _cc = match query_channel_command(bacchannel, command_id_val)
    {
        Some(cc) => cc,
        None => return (false, "could not find command".to_string())
    };
    let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
    diesel::update(channelcommands.filter(channel_bac_id.eq(&ch_id).and(command_id.eq(&cmd_id))))
        .set((has_timeout.eq(false), last_updated.eq(&ndt_now),))
        .execute(&mut connection).expect("Error updating channel command");
    return (true, "timeout disabled".to_string());
}

pub fn set_channel_command_timeout_duration(bacchannel: BACUser, command_id_val: i32, timeout_dur_val: i32) -> (bool, String)
{
    use crate::schema::channelcommands::dsl::*;
    let ch_id = &bacchannel.id.clone();
    let cmd_id = &command_id_val.clone();
    let mut connection: PgConnection = establish_connection();
    let _cc = match query_channel_command(bacchannel, command_id_val)
    {
        Some(cc) => cc,
        None => return (false, "could not find command".to_string())
    };
    let ndt_now: NaiveDateTime = chrono::offset::Local::now().naive_local();
    diesel::update(channelcommands.filter(channel_bac_id.eq(&ch_id).and(command_id.eq(&cmd_id))))
        .set((timeout_dur.eq(timeout_dur_val), last_updated.eq(&ndt_now),))
        .execute(&mut connection).expect("Error updating channel command");
    return (true, "timeout duration set".to_string());;
}

// COMMAND TIMEOUT
// CONSTRAINT commandtimeout_pkey PRIMARY KEY (channel_bac_id, user_bac_id, command_id),-
pub fn query_command_timeout(bacchannel: &BACUser, bacuser: &BACUser, command_id_val: i32) -> Option<CommandTimeout>
{
    use crate::schema::commandtimeout::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let ct_exists: bool = select(exists(commandtimeout.filter(channel_bac_id.eq(&bacchannel.id).and(user_bac_id.eq(&bacuser.id).and(command_id.eq(&command_id_val))))))
    .get_result(&mut connection).unwrap();

    if !ct_exists
    {
        // WE WILL DO NOTHING

        return None;
    }
    else
    {
        let result = commandtimeout.filter(channel_bac_id.eq(&bacchannel.id)
            .and(user_bac_id.eq(&bacuser.id)
            .and(command_id.eq(&command_id_val))))
            .first::<CommandTimeout>(&mut connection).expect("Error finding ct");
        return Some(result);
    }
}

pub fn handle_command_timeout(bacchannel: BACUser, bacuser: BACUser, command_id_val: i32, ndt_now: NaiveDateTime, timeout: i32) -> (bool, i32)
{
    use crate::schema::commandtimeout::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let channel_ct_exists: bool = select(exists(commandtimeout
        .filter(channel_bac_id.eq(&bacchannel.id)
        .and(user_bac_id.eq(&bacuser.id)
        .and(command_id.eq(&command_id_val))))))
        .get_result(&mut connection).unwrap();
    if !channel_ct_exists
    {
        // set user timeout defaults
        let cmd_id = &command_id_val.clone();
        let nct = NewCommandTimeout
        {
            channel_bac_id: &bacchannel.id, user_bac_id: &bacuser.id, command_id: cmd_id, last_command: &ndt_now
        };
        // insert
        diesel::insert_into(commandtimeout)
            .values(&nct)
            .execute(&mut connection)
            .expect("Error inserting new user command timeout");
        return (true, 0);
    }
    else
    {

        let ct = match query_command_timeout(&bacchannel, &bacuser, command_id_val)
        {
            Some(ct) => ct,
            None => panic!() // will never happen
        };
        let diff: i32 = ndt_now.signed_duration_since(ct.last_command).num_seconds().try_into().unwrap();
        if diff >= timeout
        {
            diesel::update(commandtimeout
                .filter(channel_bac_id.eq(&bacchannel.id)
                .and(user_bac_id.eq(&bacuser.id)
                .and(command_id.eq(&command_id_val)))))
                .set(last_command.eq(&ndt_now))
                .execute(&mut connection).expect("Error updating last command timestamp");
            return (true, 0);
        }
        return (false, diff);
    }
}


// INSERT SIMPLE STRING TO DATABASE
macro_rules! insert_val_to_db
{
    ($db_name:ident, $struct_t:ident, $fn_name:ident) =>
    {
        pub fn $fn_name(_name: &str)
        {
            use crate::schema::$db_name::dsl::*;
            let mut connection: PgConnection = establish_connection();
            let new_struct = $struct_t{name: _name};
            // insert
            diesel::insert_into($db_name)
            .values(&new_struct)
            .execute(&mut connection)
            .expect("Error inserting value");
        }
    };
}
insert_val_to_db!(hornedanimes, New_HornedAnime, insert_hornedanime);
insert_val_to_db!(meltys, New_Melty, insert_melty);
insert_val_to_db!(luminas, New_Lumina, insert_lumina);
insert_val_to_db!(melees, New_Melee, insert_melee);
insert_val_to_db!(sokus, New_Soku, insert_soku);
insert_val_to_db!(bbcfs, New_BBCF, insert_bbcf);
insert_val_to_db!(ggxxacplusrs, New_GGXXACPLUSR, insert_ggxxacplusr);
insert_val_to_db!(akbs, New_AKB, insert_akb);
insert_val_to_db!(vsavs, New_Vsav, insert_vsav);
insert_val_to_db!(jojos, New_Jojo, insert_jojo);
insert_val_to_db!(millions, New_Millions, insert_millions);
insert_val_to_db!(kinohackers, New_Kinohack, insert_kinohack);
insert_val_to_db!(blueayacommands, New_BACommand, insert_bacommand);

// QUERY SIMPLE STRING FROM DATABASE
macro_rules! query_string_simple
{
    ($db_name:ident, $struct_t:ident, $fn_name:ident) =>
    {
        pub fn $fn_name(q_id: i32) -> String
        {
            use crate::schema::$db_name::dsl::*;
            let mut connection: PgConnection = establish_connection();
            let result = $db_name.find(q_id).first::<$struct_t>(&mut connection).unwrap();
            return result.name;
        }
    };
}
query_string_simple!(hornedanimes, HornedAnime, query_hornedanime);
query_string_simple!(meltys, Melty, query_melty);
query_string_simple!(luminas, Lumina, query_lumina);
query_string_simple!(melees, Melee, query_melee);
query_string_simple!(sokus, Soku, query_soku);
query_string_simple!(bbcfs, BBCF, query_bbcf);
query_string_simple!(ggxxacplusrs, GGXXACPLUSR, query_ggxxacplusr);
query_string_simple!(akbs, AKB, query_akb);
query_string_simple!(vsavs, Vsav, query_vsav);
query_string_simple!(jojos, Jojo, query_jojo);
query_string_simple!(millions, Millions, query_millions);
query_string_simple!(kinohackers, Kinohack, query_kinohackers);
query_string_simple!(blueayacommands, BACommand, query_bacommand);

// GET TOTAL ITEMS IN TABLE
macro_rules! query_count_simple
{
    ($db_name:ident, $fn_name:ident) =>
    {
        pub fn $fn_name() -> i64
        {
            use crate::schema::$db_name::dsl::*;
            let mut connection: PgConnection = establish_connection();
            let result = $db_name.count().get_result(&mut connection).unwrap();
            return result;
        }
    };
}
query_count_simple!(dreamboumtweets, get_dbt_count);
query_count_simple!(nocturnedemons, get_demon_count);
query_count_simple!(hornedanimes, get_hornedanime_count);
query_count_simple!(meltys, get_melty_count);
query_count_simple!(luminas, get_lumina_count);
query_count_simple!(melees, get_melee_count);
query_count_simple!(sokus, get_soku_count);
query_count_simple!(bbcfs, get_bbcf_count);
query_count_simple!(ggxxacplusrs, get_ggxxacplusr_count);
query_count_simple!(akbs, get_akb_count);
query_count_simple!(vsavs, get_vsav_count);
query_count_simple!(jojos, get_jojo_count);
query_count_simple!(millions, get_millions_count);
query_count_simple!(kinohackers, get_kinohack_count);
query_count_simple!(blueayacommands, get_command_count);

pub fn query_cmd_to_vec() -> Vec<BACommand>
{
    use crate::schema::blueayacommands::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let results = blueayacommands
    .load::<BACommand>(&mut connection)
    .expect("Error querying commands");
    let mut out: Vec<BACommand> = Vec::new();
    for cmd in results
    {
        out.push(cmd);
    }
    return out;
}