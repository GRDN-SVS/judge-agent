extern crate actix;
extern crate actix_rt;
extern crate actix_web;
extern crate serde;
extern crate tokio;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate reqwest;
extern crate sodiumoxide;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::process;
use std::sync::Arc;

mod config;
mod crypto;
mod database;
mod handlers;
mod models;
mod services;

use database::executor::DBExecutor;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    if sodiumoxide::init().is_err() {
        eprintln!("ERROR: Could not initialize sodiumoxide correctly!");
        process::exit(1);
    }

    let addr = actix::SyncArbiter::start(2, || {
        DBExecutor::new(&env::var("DATABASE_URL").expect("No DATABASE_URL in .env"))
    });

    let encrypter = Arc::new(crypto::Encrypter::new());

    HttpServer::new(move || {
        App::new()
            .data(config::State { db: addr.clone() })
            .data(Arc::clone(&encrypter))
            .service(handlers::vote::encrypt_and_submit_vote)
            .service(handlers::public_key::box_public_key)
            .service(handlers::public_key::sign_public_key)
    })
    .bind(format!("{}:8080", &env::var("APP_URL").expect("No APP_URL in .env")))?
    .run()
    .await
}
