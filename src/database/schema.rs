table! {
    nonces (id) {
        id -> Int4,
        nonce -> Bytea,
    }
}

table! {
    votes (id) {
        id -> Int4,
        encrypted_vote -> Bytea,
        nonce_id -> Int4,
    }
}

joinable!(votes -> nonces (nonce_id));

allow_tables_to_appear_in_same_query!(nonces, votes,);
