use actix_web::{post, web, HttpResponse, Responder};
use std::sync::Arc;

use super::messages;
use crate::crypto;

use crate::{
    config::State,
    database::executor::{GetNonce, SaveVote},
    models::RequestVote,
    services::ScrutinizerService,
};

#[post("/submitVote")]
pub async fn encrypt_and_submit_vote(
    vote: web::Json<RequestVote>,
    state: web::Data<State>,
    encrypter: web::Data<Arc<crypto::Encrypter>>,
) -> impl Responder {
    let db_executor = &state.db;

    let nonce_obj = match db_executor
        .send(GetNonce {
            nonce_id: vote.nonceId,
        })
        .await
        .unwrap()
    {
        Ok(nonce_instance) => nonce_instance,
        Err(_) => {
            return HttpResponse::Ok().json(messages::error::NonceNotFound {
                code: 404,
                error: String::from("Nonce Not Found!"),
            })
        }
    };

    let scrutinizer_service = match ScrutinizerService::new().await {
        Ok(instance) => instance,
        Err(_) => {
            return HttpResponse::Ok().json(messages::error::NoScrutinizerFound {
                code: 404,
                error: String::from("Scrutinizer Not Found!"),
            })
        }
    };
    // encrypt the vote by signing it and by using the scrutinizer's public key
    let signed_vote = encrypter.sign(&vote.encryptedVote);
    let encrypted_vote = encrypter.seal(
        signed_vote,
        &nonce_obj.nonce,
        &scrutinizer_service.public_key,
    );

    let _inserted_vote = db_executor
        .send(SaveVote {
            encrypted_vote,
            nonce_id: nonce_obj.id,
            voter_public_key: vote.clientPublicKey.clone(),
        })
        .await
        .unwrap()
        .unwrap();

    HttpResponse::Ok().json(messages::success::VoteStored {
        code: 201,
        success: String::from("Vote successfully encrypted and stored"),
    })
}

// #[get("/public_key")]
// pub async fn get_key() -> impl Responder {
//     let key = " This is a key :)";

//     println!("{:?}", key);

//     HttpResponse::Ok().json(key)
// }
