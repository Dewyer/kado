INSERT INTO problem_statements(
    id,version, problem_statement)
VALUES (
    '4befd72e-13f9-11ec-ba5c-c30a7c25a635',
    'v1',
    E'#### Test Problem\nDescription goes here'
);

INSERT INTO problems(
    code_name, name, base_point_value, difficulty, max_submissions, sample_count ,problem_statement_id, available_from, available_until, is_deleted)
VALUES (
    'test-pr-1',
    'Test Problem',
    100,
    3,
    20,
    3,
    '4befd72e-13f9-11ec-ba5c-c30a7c25a635',
    '2021-09-12 20:40:00-00',
    NULL,
    false
);

INSERT INTO problem_statements(
    id,version, problem_statement)
VALUES (
           '4befd72e-13f9-11ec-ba5c-c30a7c25a622',
           'v1',
           E'#### Test Problem2\nDescription goes here'
       );

INSERT INTO problems(
    code_name, name, base_point_value, difficulty,max_submissions, sample_count , problem_statement_id, available_from, available_until, is_deleted)
VALUES (
           'test-pr-2',
           'Test Problem Almafa',
           20,
           4,
           20,
           3,
           '4befd72e-13f9-11ec-ba5c-c30a7c25a622',
           '2021-09-25 20:40:00-00',
           NULL,
           false
       );