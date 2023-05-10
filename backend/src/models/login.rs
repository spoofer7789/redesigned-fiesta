use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{options::ClientOptions, Client};
use crate::models::db::UserData;
use serde::{Deserialize, Serialize};
use actix_web::{HttpResponse, Responder, post, web};
use bson::{doc, Document};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

pub fn create_jwt_token(username: &str) -> String {
    let claims = Claims {
        sub: username.to_owned(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let encoding_key = EncodingKey::from_secret("secret_key".as_ref());
    encode(&Header::default(), &claims, &encoding_key).unwrap()
}

#[derive(Deserialize, Serialize)]
pub struct LoginData {
    identifier: String, // Either email or username
    password: String,
}

#[post("/login")]
pub async fn login_user(
    data: web::Json<LoginData>,
    db: web::Data<mongodb::Database>,
) -> impl Responder {
    let users_collection = db.collection("users");

    let user = users_collection
        .find_one(
            doc! {
                "$or": [
                    { "email": &data.identifier },
                    { "username": &data.identifier }
                ]
            },
            None,
        )
        .await
        .unwrap();

    if let Some(user_doc) = user {
        let mut user_data: UserData = bson::from_document(user_doc).unwrap();

        if user_data.password == data.password {
            let jwt_token = create_jwt_token(&user_data.username);
            
            // Remove the password field before sending the data
            
            HttpResponse::Ok().json((jwt_token, user_data))
        } else {
            HttpResponse::Unauthorized().body("Invalid password.")
        }
    } else {
        HttpResponse::NotFound().body("User not found.")
    }
}
