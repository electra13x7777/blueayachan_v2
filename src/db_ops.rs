use crate::models::
{
    New_DBTweet,
    DBTweet,
    NewBACUser,
    BACUser,
    NewRole,
    Role,
    NewBAC_User_Role,
    BAC_User_Role
};

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

pub fn get_dbt_count() -> i64
{
    use crate::schema::dreamboumtweets::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let count: i64 = dreamboumtweets.count().get_result(&mut connection).unwrap();
    return count;
}

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