diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        token -> Varchar,
    }
}

diesel::table! {
    authtokens (value) {
        value -> Varchar,
        created -> Timestamp,
        user_id -> Int4,
    }
}
