
use actix_web::{ App, HttpServer, middleware};
use actix_web::web::Data;
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{options::ClientOptions, Client};
use std::io;
use env_logger;
mod api;
mod models;
mod utils;
mod controllers;
use crate::api::routes::ws_index; // Include get_user_data_route
use crate::models::db::register_user;
use crate::models::init::get_profile;
use crate::models::login::login_user;
use crate::utils::ipfssync::web3_login;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("my_db");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .wrap(middleware::Logger::default()) // Add Logger middleware
            .service(login_user) 
            .service(web3_login)
            .service(ws_index) 
            .service(get_profile)
            .service(register_user)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}




// mod api;
// mod controllers;
// mod utils;
// use actix::prelude::*;
// use actix_web::{get,web, App, HttpRequest, HttpResponse, HttpServer};
// use actix_web_actors::ws;
// use api::routes::{ws_route, web3_login, create_account, login};
// use serde::{Deserialize, Serialize};
// use std::sync::Mutex;
// use controllers::routes::add_user;
// use api::state::AppState;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     // Initialize AppState with one Vec<UserData> field
//     let app_data = web::Data::new(AppState {
//         user_data: Mutex::new(vec![]),
//     });
//     HttpServer::new(move || {
//         App::new().app_data(app_data.clone())
//         .service(index)
//          .route("/create_account", web::post().to(create_account))
//          .route("/login", web::post().to(login))
//          //.route("/make_post", web::post().to(make_post))
//         .route("/ws/", web::get().to(ws_route))
//         .route("/add_user", web::post().to(add_user))
//         .route("/web3_login", web::post().to(web3_login))
//     })
//     .bind(("127.0.0.1", 3000))?
//     .run().await
// }