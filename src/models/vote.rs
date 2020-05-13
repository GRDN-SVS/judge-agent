use crate::database::schema::votes;
use serde::{Deserialize, Serialize};

/// Representation of a Vote already stored inside the database
#[derive(Queryable, Serialize)]
pub struct Vote {
    pub id: i32,
    pub encrypted_vote: Vec<u8>,
    pub nonce_id: i32,
}

/// Struct used to insert a new Vote inside the database
#[derive(Insertable)]
#[table_name = "votes"]
pub struct InsertableVote {
    /// Encrypted vote to be stored in database
    pub encrypted_vote: Vec<u8>,
    pub nonce_id: i32,
}

/// Representation of a Vote that comes from an HTTP request
/// as a JSON string
#[derive(Deserialize)]
pub struct RequestVote {
    pub nonceId: i32,
    pub encryptedVote: Vec<u8>,
    pub clientPublicKey: Vec<u8>,
}
