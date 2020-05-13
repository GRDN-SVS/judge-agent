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
use sodiumoxide::crypto::box_::gen_keypair;
use std::env;
use std::process;

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

    if let Err(_) = sodiumoxide::init() {
        eprintln!("ERROR: Could not initialize sodiumoxide correctly!");
        process::exit(1);
    }

    let addr = actix::SyncArbiter::start(2, || {
        DBExecutor::new(&env::var("DATABASE_URL").expect("No DATABASE_URL in .env"))
    });

    HttpServer::new(move || {
        App::new()
            .data(config::State { db: addr.clone() })
            .data(crypto::Encrypter::new())
            .service(handlers::vote::encrypt_and_submit_vote)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
