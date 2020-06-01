use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;

use super::messages;
use crate::crypto;

#[get("/boxPublicKey")]
pub async fn box_public_key(encrypter: web::Data<Arc<crypto::Encrypter>>) -> impl Responder {
    let key = messages::success::PublicKey {
        code: 200,
        key: &encrypter.box_public_key.0.to_vec(),
    };

    HttpResponse::Ok().json(key)
}

#[get("/signPublicKey")]
pub async fn sign_public_key(encrypter: web::Data<Arc<crypto::Encrypter>>) -> impl Responder {
    let key = messages::success::PublicKey {
        code: 200,
        key: &encrypter.sign_public_key.0.to_vec(),
    };

    HttpResponse::Ok().json(key)
}