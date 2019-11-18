table! {
    users_sessions (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        token -> Varchar,
    }
}
