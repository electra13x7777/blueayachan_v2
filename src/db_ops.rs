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
    count,
    exists
};
use chrono::
{
    DateTime,
    NaiveDateTime
};
use diesel::sql_query;

///////////////////////////////////////////////////////////////////////////////
//                           USER, ROLES, USERROLES                          //
///////////////////////////////////////////////////////////////////////////////

// called when a new user sends a valid request to execute a commands
pub fn handle_bac_user_in_db(user_nick_str: String)
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
        {user_nick: &user_nick_lower, num_commands: &first_command, date_added: &nt_now};
        // insert
        diesel::insert_into(blueayachanuser)
        .values(&new_bac_user)
        .execute(&mut connection)
        .expect("Error inserting new user");
    }
    else // ALREADY IN DATABASE
    {
        let updated_row = diesel::update(blueayachanuser.filter(user_nick.eq(user_nick_lower)))
        .set(num_commands.eq(num_commands+1)).execute(&mut connection);
    }
}

pub fn query_user_data(user_nick_str: String) -> BACUser
{
    use crate::schema::blueayachanuser::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let user_nick_lower: String = user_nick_str.to_lowercase();
    let result = blueayachanuser.filter(user_nick.eq(&user_nick_lower)).first::<BACUser>(&mut connection).expect("Oh no!");
    return result;
}

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
/*
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
            last_demon_id: &ld_id, last_demon_rarity: &ld_r
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
/*
query_string_simple!(meltys, New_Melty, query_melty);
query_string_simple!(luminas, New_Lumina, query_lumina);
query_string_simple!(melees, New_Melee, query_melee);
query_string_simple!(sokus, New_Soku, query_soku);
query_string_simple!(bbcfs, New_BBCF, query_bbcf);
query_string_simple!(ggxxacplusrs, New_GGXXACPLUSR, query_ggxxacplusr);
query_string_simple!(akbs, New_AKB, query_akb);
query_string_simple!(vsavs, New_Vsav, query_vsav);
*/

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