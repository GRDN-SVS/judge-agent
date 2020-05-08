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
    }
}

allow_tables_to_appear_in_same_query!(nonces, votes,);
