CREATE TABLE submissions
(
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v1(),
    owner_id UUID NOT NULL,
    problem_id UUID NOT NULL,
    seed INT NOT NULL,
    test_count INT NOT NULL,
    correct BOOL NULL,
    proof_file_path VARCHAR NULL,
    proof_file_original_name VARCHAR NULL,
    sample_index INT NULL,
    started_at TIMESTAMP NOT NULL,
    finished_at TIMESTAMP NULL,
    FOREIGN KEY (problem_id) REFERENCES problems (id) ON DELETE CASCADE,
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE submission_tests
(
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v1(),
    submission_id UUID NOT NULL,
    class VARCHAR NOT NULL,
    input TEXT NOT NULL,
    output TEXT NULL,
    correct BOOL NULL,
    started_at TIMESTAMP NOT NULL,
    finished_at TIMESTAMP NULL,
    FOREIGN KEY (submission_id) REFERENCES submissions (id) ON DELETE CASCADE
);