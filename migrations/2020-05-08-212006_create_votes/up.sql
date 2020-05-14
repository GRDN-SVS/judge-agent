CREATE TABLE votes (
    id SERIAL PRIMARY KEY,
    encrypted_vote BYTEA NOT NULL,
    nonce_id Integer NOT NULL REFERENCES nonces(id)
);