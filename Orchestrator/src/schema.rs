// @generated automatically by Diesel CLI.

diesel::table! {
    waifu_databases (id) {
        id -> Int4,
        hashed_secret -> Text,
        waifu_name -> Text,
        db_name -> Text,
        db_user -> Text,
        db_password -> Text,
        created_at -> Timestamp,
    }
}
