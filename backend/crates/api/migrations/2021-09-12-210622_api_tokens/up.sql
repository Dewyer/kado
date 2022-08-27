CREATE TABLE api_tokens
(
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v1(),
    token VARCHAR NOT NULL UNIQUE,
    owner_id UUID NOT NULL,
    is_deleted BOOL NOT NULL DEFAULT FALSE,
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);