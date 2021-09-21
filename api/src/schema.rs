table! {
    api_tokens (id) {
        id -> Uuid,
        token -> Varchar,
        owner_id -> Uuid,
        is_deleted -> Bool,
    }
}

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
        max_submissions -> Int4,
        sample_count -> Int4,
        problem_statement_id -> Uuid,
        available_from -> Nullable<Timestamp>,
        available_until -> Nullable<Timestamp>,
        is_deleted -> Bool,
    }
}

table! {
    submission_tests (id) {
        id -> Uuid,
        submission_id -> Uuid,
        class -> Varchar,
        input -> Text,
        output -> Nullable<Text>,
        correct -> Nullable<Bool>,
        started_at -> Timestamp,
        finished_at -> Nullable<Timestamp>,
    }
}

table! {
    submissions (id) {
        id -> Uuid,
        owner_id -> Uuid,
        problem_id -> Uuid,
        seed -> Int4,
        test_count -> Int4,
        correct -> Nullable<Bool>,
        proof_file_path -> Nullable<Varchar>,
        proof_file_original_name -> Nullable<Varchar>,
        sample_index -> Nullable<Int4>,
        started_at -> Timestamp,
        finished_at -> Nullable<Timestamp>,
    }
}

table! {
    teams (id) {
        id -> Uuid,
        name -> Varchar,
        join_code -> Varchar,
        points -> Int8,
        participate_in_leaderboards -> Bool,
        last_gained_points_at -> Nullable<Timestamp>,
        owner_user -> Nullable<Uuid>,
        is_deleted -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        authenticator -> Varchar,
        participate_in_leaderboards -> Bool,
        individual_points -> Int8,
        last_gained_points_at -> Nullable<Timestamp>,
        is_active -> Bool,
        is_admin -> Bool,
        created_at -> Timestamp,
        team_id -> Nullable<Uuid>,
    }
}

joinable!(api_tokens -> users (owner_id));
joinable!(problems -> problem_statements (problem_statement_id));
joinable!(submission_tests -> submissions (submission_id));
joinable!(submissions -> problems (problem_id));
joinable!(submissions -> users (owner_id));
joinable!(teams -> users (owner_user));

allow_tables_to_appear_in_same_query!(
    api_tokens,
    problem_statements,
    problems,
    submission_tests,
    submissions,
    teams,
    users,
);
