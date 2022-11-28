// @generated automatically by Diesel CLI.

diesel::table! {
    blueayachanuser (id) {
        id -> Int4,
        user_nick -> Varchar,
        num_commands -> Int4,
        date_added -> Nullable<Timestamp>,
    }
}

diesel::table! {
    blueayachanuser_roles (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        role_id -> Nullable<Int4>,
        created -> Nullable<Timestamp>,
    }
}

diesel::table! {
    dreamboumtweets (id) {
        id -> Int4,
        tweet -> Varchar,
        tweet_date -> Varchar,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        role_name -> Nullable<Varchar>,
        date_added -> Nullable<Timestamp>,
    }
}

diesel::joinable!(blueayachanuser_roles -> blueayachanuser (user_id));
diesel::joinable!(blueayachanuser_roles -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    blueayachanuser,
    blueayachanuser_roles,
    dreamboumtweets,
    roles,
);
