use serde::Serialize;

#[derive(Serialize)]
pub struct NonceNotFoundError {
    pub code: i32,
    pub error: String,
}

#[derive(Serialize)]
pub struct NoScrutinizerFoundError {
    pub code: i32,
    pub error: String,
}

#[derive(Serialize)]
pub struct VoteStoredSuccess {
    pub code: i32,
    pub success: String,
}
