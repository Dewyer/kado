CREATE
EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users
(
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v1(),
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    authenticator VARCHAR NOT NULL,
    participate_in_leaderboards BOOL NOT NULL DEFAULT false,
    individual_points BIGINT NOT NULL,
    last_gained_points_at TIMESTAMP NULL,
    is_active BOOL NOT NULL DEFAULT TRUE,
    is_admin BOOL NOT NULL DEFAULT FALSE
);