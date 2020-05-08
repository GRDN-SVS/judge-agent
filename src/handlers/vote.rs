use actix_web::{post, web, HttpResponse, Responder};
use sodiumoxide::crypto::box_::gen_nonce;


use crate::{config::State, database::executor::SaveVote, models::RequestVote};

#[post("/submitVote")]
pub async fn encrypt_and_submit_vote(vote: web::Json<RequestVote>, state: web::Data<State>) -> impl Responder {

    let db_executor = &state.db;

    // encrypt the vote...


    HttpResponse::Ok().json(inserted_vote)
}

#[get("/public_key")]
pub async fn get_key() -> impl Responder {
    let key =" This is a key :)";

    println!("{:?}", key);

    HttpResponse::Ok().json(key)
}
