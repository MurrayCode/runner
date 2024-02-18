// @generated automatically by Diesel CLI.

diesel::table! {
    records (id) {
        id -> Integer,
        user_id -> Integer,
        run_id -> Integer,
        distance -> Integer,
        duration -> Integer,
    }
}

diesel::table! {
    runs (id) {
        id -> Integer,
        distance -> Integer,
        duration -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
    }
}

diesel::joinable!(records -> runs (run_id));
diesel::joinable!(records -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    records,
    runs,
    users,
);
