INSERT INTO problem_statements(id, version, problem_statement)
VALUES ('4befd72e-13f9-11ec-4242-c30a7c25a635',
        'v1',
        E'#### Sanity check\n');

INSERT INTO problems(code_name, name, base_point_value, difficulty, max_submissions, sample_count, problem_statement_id,
                     available_from, available_until, is_deleted)
VALUES ('sanity-check',
        'Sanity Check',
        20,
        1,
        30,
        1,
        '4befd72e-13f9-11ec-4242-c30a7c25a635',
        '2021-09-20 00:00:00-00',
        NULL,
        false);