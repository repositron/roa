// @generated automatically by Diesel CLI.

diesel::table! {
    orders (id) {
        id -> Uuid,
        table_id -> Int4,
        item -> Varchar,
        duration -> Int4,
        expire_at -> Timestamp,
        created_at -> Timestamp,
    }
}
