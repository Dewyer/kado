INSERT INTO problem_statements(id, version, problem_statement)
VALUES ('4befd72e-13f9-11ec-4242-c30a7c25a666',
        'v1',
        E'#### Maze\n');

INSERT INTO problems(code_name, name, base_point_value, difficulty, max_submissions, sample_count, problem_statement_id,
                     available_from, available_until, is_deleted)
VALUES ('maze',
        'Maze',
        50,
        2,
        10,
        2,
        '4befd72e-13f9-11ec-4242-c30a7c25a666',
        '2021-09-28 00:00:00-00',
        NULL,
        false);