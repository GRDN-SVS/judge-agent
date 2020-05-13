use diesel::sql_types::{Binary, Integer};

/// Representation of an Nonce already stored inside the database
#[derive(Queryable, QueryableByName, Clone)]
pub struct Nonce {
    #[sql_type = "Integer"]
    pub id: i32,
    /// Nonce to be used on vote encryption.
    #[sql_type = "Binary"]
    pub nonce: Vec<u8>,
}
