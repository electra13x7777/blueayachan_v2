use diesel::prelude::*;
use crate::schema::
{
    blueayachanuser,
    blueayachanuser_roles,
    dreamboumtweets,
    roles
};
use chrono::NaiveDateTime;

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
//                           USER, ROLES, USERROLES                          //
///////////////////////////////////////////////////////////////////////////////


#[derive(Insertable)]//, Identifiable)]
#[diesel(table_name = blueayachanuser)]
pub struct NewBACUser<'a>
{
    pub user_nick: &'a str,
    pub num_commands: &'a i32,
    pub date_added: &'a NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = blueayachanuser)]
pub struct BACUser
{
    pub id: i32,
    pub user_nick: String,
    pub num_commands: i32,
    pub date_added: NaiveDateTime,
}
/*
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
#[diesel(belongs_to(BACUser, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[derive(Insertable, Associations)]
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