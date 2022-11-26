use crate::models::{New_DBTweet, DBTweet};
use crate::db_connect::establish_connection;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::dsl::count;
use diesel::sql_query;

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

pub fn get_dbt_count() -> i64
{
    use crate::schema::dreamboumtweets::dsl::*;
    let mut connection: PgConnection = establish_connection();
    let count: i64 = dreamboumtweets.count().get_result(&mut connection).unwrap();
    return count;
}