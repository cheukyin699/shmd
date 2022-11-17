// @generated automatically by Diesel CLI.

diesel::table! {
    media (id) {
        id -> Int4,
        title -> Text,
        artist -> Nullable<Text>,
        album -> Nullable<Text>,
        location -> Text,
    }
}
