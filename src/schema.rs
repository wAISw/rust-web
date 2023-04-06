// @generated automatically by Diesel CLI.

diesel::table! {
    actions_queue (id) {
        id -> Varchar,
        action_type -> Varchar,
        data -> Varchar,
        created_at -> Timestamptz,
    }
}
