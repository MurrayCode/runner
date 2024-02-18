table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    runs (id) {
        id -> Int4,
        user_id -> Int4,
        distance -> Int4,
        duration -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    records (id) {
        id -> Int4,
        user_id -> Int4,
        run_id -> Int4,
        distance -> Int4,
        duration -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(runs -> users (user_id));
joinable!(records -> runs (run_id));
joinable!(records -> users (user_id));

allow_tables_to_appear_in_same_query!(users, runs, records,);
