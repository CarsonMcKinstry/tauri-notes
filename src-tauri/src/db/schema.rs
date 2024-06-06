// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Text,
        title -> Text,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        active -> Bool,
    }
}
