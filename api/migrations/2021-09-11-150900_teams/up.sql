CREATE TABLE teams
(
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v1(),
    name VARCHAR NOT NULL UNIQUE,
    join_code VARCHAR NOT NULL,
    points BIGINT NOT NULL,
    owner_user UUID NULL,
    is_deleted BOOL NOT NULL,
    FOREIGN KEY (owner_user) REFERENCES users (id) ON DELETE CASCADE
);