table! {
    teams (id) {
        id -> Uuid,
        name -> Varchar,
        join_code -> Varchar,
        owner_user -> Nullable<Uuid>,
        is_deleted -> Bool,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        authenticator -> Varchar,
        participate_in_leaderboards -> Bool,
        is_active -> Bool,
        is_admin -> Bool,
        team_id -> Nullable<Uuid>,
    }
}

joinable!(teams -> users (owner_user));

allow_tables_to_appear_in_same_query!(
    teams,
    users,
);
