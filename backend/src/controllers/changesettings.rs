use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::models::db::UserData;

#[derive(Deserialize)]
pub struct UserInfo {
    username: String,
}

// Handler for GET /users/{username} request
// Handler for GET /users/{username} request
pub async fn get_user(info: web::Path<UserInfo>) -> impl Responder {
    // Fetch the user from the database using the username
    // For now, we'll just return a dummy user
    HttpResponse::Ok().body(format!("User: {}", info.username))
}


// Handler for POST /users request
pub async fn create_user(user: web::Json<UserData>) -> impl Responder {
    // Create a new user in the database using the provided user data
    // For now, we'll just echo back the user data
    HttpResponse::Ok().json(user.into_inner())
}
