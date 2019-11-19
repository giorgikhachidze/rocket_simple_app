table! {
    users (id) {
        id -> Unsigned<Bigint>,
        email -> Varchar,
        password -> Varchar,
    }
}

table! {
    users_sessions (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        token -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    users_sessions,
);
