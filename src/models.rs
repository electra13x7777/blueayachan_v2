use diesel::prelude::*;
use crate::schema::*;
use chrono::NaiveDateTime;

///////////////////////////////////////////////////////////////////////////////
//                           USER, ROLES, USERROLES                          //
///////////////////////////////////////////////////////////////////////////////


#[derive(Insertable)]//, Identifiable)]
#[diesel(table_name = blueayachanuser)]
pub struct NewBACUser<'a>
{
    pub user_nick: &'a str,
    pub num_commands: &'a i32,
    pub date_added: &'a NaiveDateTime,
    pub twitch_id: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = blueayachanuser)]
pub struct BACUser
{
    pub id: i32,
    pub user_nick: String,
    pub num_commands: i32,
    pub date_added: NaiveDateTime,
    pub twitch_id: String,
}
/*
#[derive(Insertable)]//, Identifiable)]
#[diesel(table_name = bac_twitch_id)]
pub struct NewBACTwitchId<'a>
{
    pub user_id: &'a i32,
    pub twitch_id: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = bac_twitch_id)]
pub struct BACTwitchId
{
    pub id: i32,
    pub user_id: i32,
    pub twitch_id: String,
}

last_pasta TIMESTAMP DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
last_pic TIMESTAMP DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
*/

#[derive(Insertable)]//, Identifiable)]
#[diesel(table_name = roles)]
pub struct NewRole<'a>
{
    pub role_name: &'a str,
    pub date_added: &'a NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = roles)]
pub struct Role
{
    pub id: i32,
    pub role_name: String,
    pub date_added: NaiveDateTime,
}


//#[diesel(belongs_to(i32, foreign_key = user_id))]
//#[diesel(belongs_to(roles, foreign_key = role_id))]
#[derive(Insertable, Associations)]
#[diesel(belongs_to(BACUser, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(table_name = blueayachanuser_roles)]
pub struct NewBAC_User_Role<'a>
{
    pub user_id: &'a i32,
    pub role_id: &'a i32,
    pub created: &'a NaiveDateTime
}

#[derive(Queryable, Selectable, Associations)]
//#[diesel(belongs_to(blueayachanuser, foreign_key = user_id))]
//#[diesel(belongs_to(roles, foreign_key = role_id))]
#[diesel(belongs_to(BACUser, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(table_name = blueayachanuser_roles)]
pub struct BAC_User_Role
{
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created: NaiveDateTime,
}

///////////////////////////////////////////////////////////////////////////////
//                              DREAMBOUMTWEETS                              //
///////////////////////////////////////////////////////////////////////////////

#[derive(Insertable)]
#[diesel(table_name = dreamboumtweets)]
pub struct New_DBTweet<'a>
{
    pub tweet: &'a str,
    pub tweet_date: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = dreamboumtweets)]
pub struct DBTweet
{
    pub id: i32,
    pub tweet: String,
    pub tweet_date: String,
}

///////////////////////////////////////////////////////////////////////////////
//                           GACHA COMMANDS RELATED                          //
///////////////////////////////////////////////////////////////////////////////

#[derive(Insertable)]
#[diesel(table_name = nocturnedemons)]
pub struct New_NDemon<'a>
{
    pub demon_name: &'a str,
    pub demon_img_link: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = nocturnedemons)]
pub struct NDemon
{
    pub id: i32,
    pub demon_name: String,
    pub demon_img_link: String,
}

#[derive(Insertable)]
#[diesel(belongs_to(blueayachanuser, foreign_key = user_id))]
#[diesel(table_name = bac_user_demons)]
pub struct New_SavedNDemon<'a>
{
    pub user_id: &'a i32, // BACUser
    // only updates when saved
    pub saved_demon_id: &'a i32,
    pub saved_demon_rarity: &'a i32,
    // updated every time
    pub last_demon_id: &'a i32,
    pub last_demon_rarity: &'a i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = bac_user_demons)]
pub struct SavedNDemon
{
    pub id: i32,
    pub user_id: i32,
    //pub saved_demon_name: String,
    pub saved_demon_id: i32,
    pub saved_demon_rarity: i32,
    //pub last_demon_name: String,
    pub last_demon_id: i32,
    pub last_demon_rarity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = pictimeout)]
pub struct NewPicTimeout<'a>
{
    pub user_id: &'a i32, // foreign_key from blueayachanuser
    pub last_pic: &'a NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = pictimeout)]
pub struct PicTimeout
{
    pub id: i32,
    pub user_id: i32,
    pub last_pic: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = botchannels)]
pub struct NewBotChannel<'a>
{
    pub channel_name: &'a str, // foreign_key from botchannels
    pub channel_twitch_id: &'a str,
    pub last_updated: &'a NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = botchannels)]
pub struct BotChannel
{
    pub id: i32,
    pub channel_name: String, // foreign_key from botchannels
    pub channel_twitch_id: String,
    pub last_updated: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = channelcommands)]
pub struct NewChannelCommands<'a>
{
    //pub channel_twitch_id: &'a str, // foreign_key from botchannels
    pub channel_bac_id: &'a i32,
    pub command_id: &'a i32, // foreign_key from blueayacommands
    pub is_active: &'a bool,
    pub is_broadcaster_only: &'a bool,
    pub is_mod_only: &'a bool,
    pub has_timeout: &'a bool,
    pub timeout_dur: &'a i32,
    //pub num_used: &'a i32, // WILL PUT THIS IN ITS OWN TABLE
    pub last_updated: &'a NaiveDateTime,
}

#[derive(Queryable, Selectable)]
//#[primary_key(channel_bac_id, command_id)]
#[diesel(table_name = channelcommands)]
pub struct ChannelCommands
{
    pub id: i32,
    //pub channel_twitch_id: String, // foreign_key from botchannels
    pub channel_bac_id: i32,
    pub command_id: i32, // foreign_key from blueayacommands
    pub is_active: bool,
    pub is_broadcaster_only: bool,
    pub is_mod_only: bool,
    pub has_timeout: bool,
    pub timeout_dur: i32,
    //pub num_used: i32,
    pub last_updated: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = commandtimeout)]
pub struct NewCommandTimeout<'a>
{
    //pub channel_twitch_id: &'a str, // foreign_key from botchannels
    //pub user_twitch_id: &'a str,
    pub channel_bac_id: &'a i32,
    pub user_bac_id: &'a i32,
    pub command_id: &'a i32, // foreign_key from blueayacommands
    pub last_command: &'a NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = commandtimeout)]
pub struct CommandTimeout
{
    pub id: i32,
    //pub channel_twitch_id: String, // foreign_key from botchannels
    //pub user_twitch_id: String,
    pub channel_bac_id: i32,
    pub user_bac_id: i32,
    pub command_id: i32, // foreign_key from blueayacommands
    pub last_command: NaiveDateTime,
}

// GENERATE DB ENDPOINTS
macro_rules! generate_simple_db_structs
{
    ($db_name:ident,
    $new_struct_t:ident,
    $struct_t:ident,
    $gen_l:lifetime) =>
    {
        #[derive(Insertable)]
        #[diesel(table_name = $db_name)]
        pub struct $new_struct_t<$gen_l>
        {
            pub name: &$gen_l str,
        }
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = $db_name)]
        pub struct $struct_t
        {
            pub id: i32,
            pub name: String,
        }
    };
}
generate_simple_db_structs!(hornedanimes, New_HornedAnime, HornedAnime, 'a);
generate_simple_db_structs!(meltys, New_Melty, Melty, 'a);
generate_simple_db_structs!(luminas, New_Lumina, Lumina, 'a);
generate_simple_db_structs!(melees, New_Melee, Melee, 'a);
generate_simple_db_structs!(sokus, New_Soku, Soku, 'a);
generate_simple_db_structs!(bbcfs, New_BBCF, BBCF, 'a);
generate_simple_db_structs!(ggxxacplusrs, New_GGXXACPLUSR, GGXXACPLUSR, 'a);
generate_simple_db_structs!(akbs, New_AKB, AKB, 'a);
generate_simple_db_structs!(vsavs, New_Vsav, Vsav, 'a);
generate_simple_db_structs!(jojos, New_Jojo, Jojo, 'a);
generate_simple_db_structs!(millions, New_Millions, Millions, 'a);
generate_simple_db_structs!(kinohackers, New_Kinohack, Kinohack, 'a);
generate_simple_db_structs!(blueayacommands, New_BACommand, BACommand, 'a);