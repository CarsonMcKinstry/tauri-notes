diesel::table! {
    notes (id) {
        id -> Binary,
        title -> Text,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        active -> Bool
    }
}
