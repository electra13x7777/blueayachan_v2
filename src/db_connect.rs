use diesel::pg::PgConnection;
/*
use diesel_async::
{
    AsyncConnection,
    AsyncPgConnection
};
*/
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// single threaded connections
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// connections in a multi-threaded routine
/*
pub async fn establish_connection_async() -> AsyncPgConnection
{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return AsyncPgConnection::establish(&database_url).await
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}
*/
