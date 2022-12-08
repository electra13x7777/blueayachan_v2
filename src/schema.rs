// @generated automatically by Diesel CLI.

diesel::table! {
    akbs (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    bac_user_demons (id) {
        id -> Int4,
        user_id -> Int4,
        saved_demon_id -> Int4,
        saved_demon_rarity -> Int4,
        last_demon_id -> Int4,
        last_demon_rarity -> Int4,
    }
}

diesel::table! {
    bbcfs (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    blueayachanuser (id) {
        id -> Int4,
        user_nick -> Varchar,
        num_commands -> Int4,
        date_added -> Timestamp,
    }
}

diesel::table! {
    blueayachanuser_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created -> Timestamp,
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
    ggxxacplusrs (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    hornedanimes (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    luminas (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    melees (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    meltys (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    nocturnedemons (id) {
        id -> Int4,
        demon_name -> Varchar,
        demon_img_link -> Varchar,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        role_name -> Varchar,
        date_added -> Timestamp,
    }
}

diesel::table! {
    sokus (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    vsavs (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(bac_user_demons -> blueayachanuser (user_id));
diesel::joinable!(blueayachanuser_roles -> blueayachanuser (user_id));
diesel::joinable!(blueayachanuser_roles -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    akbs,
    bac_user_demons,
    bbcfs,
    blueayachanuser,
    blueayachanuser_roles,
    dreamboumtweets,
    ggxxacplusrs,
    hornedanimes,
    luminas,
    melees,
    meltys,
    nocturnedemons,
    roles,
    sokus,
    vsavs,
);
