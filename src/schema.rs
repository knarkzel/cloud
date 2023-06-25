// @generated automatically by Diesel CLI.

diesel::table! {
    wasm (hash) {
        hash -> Text,
        binary -> Binary,
        title -> Text,
        description -> Text,
    }
}
