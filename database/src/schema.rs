// @generated automatically by Diesel CLI.

diesel::table! {
    areas (id) {
        id -> Int4,
        continent_id -> Int4,
        shortname -> Varchar,
        displayname -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    characters (id) {
        id -> Int4,
        user_id -> Int4,
        shortname -> Varchar,
        description -> Varchar,
        current_room_id -> Int4,
    }
}

diesel::table! {
    continents (id) {
        id -> Int4,
        plane_id -> Int4,
        shortname -> Varchar,
        displayname -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    environments (id) {
        id -> Int4,
        shortname -> Varchar,
        displayname -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    exits (id) {
        id -> Int4,
        from_room_id -> Int4,
        to_room_id -> Int4,
        hidden -> Bool,
    }
}

diesel::table! {
    planes (id) {
        id -> Int4,
        shortname -> Varchar,
        displayname -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    rooms (id) {
        id -> Int4,
        area_id -> Int4,
        shortname -> Varchar,
        displayname -> Varchar,
        description -> Varchar,
        environment_id -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        autologin -> Nullable<Int4>,
        administrator -> Bool,
    }
}

diesel::joinable!(areas -> continents (continent_id));
diesel::joinable!(characters -> rooms (current_room_id));
diesel::joinable!(characters -> users (user_id));
diesel::joinable!(continents -> planes (plane_id));
diesel::joinable!(rooms -> areas (area_id));

diesel::allow_tables_to_appear_in_same_query!(
    areas,
    characters,
    continents,
    environments,
    exits,
    planes,
    rooms,
    users,
);
