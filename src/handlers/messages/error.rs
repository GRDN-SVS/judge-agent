use serde::Serialize;

#[derive(Serialize)]
pub struct NonceNotFound {
    pub code: i32,
    pub error: String,
}

#[derive(Serialize)]
pub struct NoScrutinizerFound {
    pub code: i32,
    pub error: String,
}
