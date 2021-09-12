table! {
    problem_statements (id) {
        id -> Uuid,
        version -> Nullable<Varchar>,
        problem_statement -> Text,
    }
}

table! {
    problems (id) {
        id -> Uuid,
        code_name -> Varchar,
        name -> Varchar,
        base_point_value -> Int8,
        difficulty -> Int4,
        problem_statement_id -> Uuid,
        available_from -> Nullable<Timestamp>,
        available_until -> Nullable<Timestamp>,
        is_deleted -> Bool,
    }
}

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

joinable!(problems -> problem_statements (problem_statement_id));
joinable!(teams -> users (owner_user));

allow_tables_to_appear_in_same_query!(
    problem_statements,
    problems,
    teams,
    users,
);
