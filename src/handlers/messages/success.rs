use serde::Serialize;

#[derive(Serialize)]
pub struct VoteStored {
    pub code: i32,
    pub success: String,
}

#[derive(Serialize)]
pub struct PublicKey<'a> {
    pub code: i32,
    pub key: &'a Vec<u8>,
}
