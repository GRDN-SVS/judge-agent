CREATE TABLE votes (
    id SERIAL PRIMARY KEY,
    encrypted_vote BYTEA NOT NULL
);