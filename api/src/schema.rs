table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        authenticator -> Varchar,
        participate_in_leaderboards -> Bool,
        is_active -> Bool,
        is_admin -> Bool,
    }
}
