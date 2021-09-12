CREATE TABLE problem_statements
(
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v1(),
    version VARCHAR NULL,
    problem_statement TEXT NOT NULL DEFAULT ''
);

CREATE TABLE problems
(
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v1(),
    code_name VARCHAR NOT NULL UNIQUE,
    name VARCHAR NOT NULL UNIQUE,
    base_point_value BIGINT NOT NULL,
    difficulty INT NOT NULL,
    problem_statement_id UUID NOT NULL,
    available_from timestamp NULL,
    available_until timestamp NULL,
    is_deleted BOOL NOT NULL,
    FOREIGN KEY (problem_statement_id) REFERENCES problem_statements (id) ON DELETE CASCADE
);