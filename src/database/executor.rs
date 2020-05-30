use actix::prelude::*;
use diesel::pg::PgConnection;
use diesel::query_dsl::RunQueryDsl;
use diesel::Connection;

use crate::models;

use super::schema::votes;

/// An [actor](https://en.wikipedia.org/wiki/Actor_model)
/// that connects to a postgres database, being the only one
/// in charge of interacting directly with it.
pub struct DBExecutor(PgConnection);

impl DBExecutor {
    pub fn new(connection_string: &str) -> DBExecutor {
        DBExecutor(
            diesel::pg::PgConnection::establish(connection_string)
                .expect("Could not Connect to the database"),
        )
    }
}

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}

/// Message that can be sent to the DBExecutor to
/// tell it to save a vote to the database.
pub struct SaveVote {
    pub encrypted_vote: Vec<u8>,
    pub nonce_id: i32,
    pub voter_public_key: Vec<u8>,
}

impl Message for SaveVote {
    type Result = Result<models::Vote, diesel::result::Error>;
}

impl Handler<SaveVote> for DBExecutor {
    type Result = Result<models::Vote, diesel::result::Error>;

    fn handle(&mut self, msg: SaveVote, _: &mut Self::Context) -> Self::Result {
        let insertable_vote = models::InsertableVote {
            encrypted_vote: msg.encrypted_vote,
            nonce_id: msg.nonce_id,
            voter_public_key: msg.voter_public_key,
        };

        let vote_obj = diesel::insert_into(votes::table)
            .values(&insertable_vote)
            .get_result(&self.0)?;

        Ok(vote_obj)
    }
}

/// Message that can be sent to the DBExecutor to
/// tell it to save a vote to the database
pub struct GetNonce {
    pub nonce_id: i32,
}

impl Message for GetNonce {
    type Result = Result<models::Nonce, diesel::result::Error>;
}

impl Handler<GetNonce> for DBExecutor {
    type Result = Result<models::Nonce, diesel::result::Error>;

    fn handle(&mut self, msg: GetNonce, _: &mut Self::Context) -> Self::Result {
        let query_string = format!("SELECT * FROM nonces WHERE id = {}", &msg.nonce_id);
        let nonce_ref: &models::Nonce = &diesel::sql_query(query_string).load(&self.0)?[0];
        let nonce_obj: models::Nonce = nonce_ref.clone();

        Ok(nonce_obj)
    }
}
