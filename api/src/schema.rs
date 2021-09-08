table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        authenticator -> Varchar,
        is_active -> Bool,
        is_admin -> Bool,
    }
}
