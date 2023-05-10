use actix_web::{post, web, HttpResponse,  Responder, middleware, HttpRequest};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use bson::{doc, Document};
use reqwest;
use wallet_gen::generate_zcash_wallet;
use super::posts::Post;

#[derive(Debug,Deserialize, Serialize)]
 pub struct UserData {
    email: String,
   pub(crate) username: String,
    pub password: String,
    #[serde(default)]
    wallets: Vec<WalletType>,
    #[serde(default)]
    pub posts: Vec<Document>,
    #[serde(default)]
    subscriptions: Vec<String>,
    #[serde(default)]
    interests: Vec<String>,
    #[serde(default)]
    interactions: Vec<String>,
    #[serde(default)]
    contracts: Vec<WalletType>,
}

#[post("/register")]
pub async fn register_user(
    data: web::Json<UserData>,
    db: web::Data<mongodb::Database>,
) -> impl Responder {
    let users_collection = db.collection("users");

    // Check if the email or username has been taken
    let email_taken = users_collection
        .find_one(doc! { "email": &data.email }, None)
        .await
        .unwrap()
        .is_some();
    let username_taken = users_collection
        .find_one(doc! { "username": &data.username }, None)
        .await
        .unwrap()
        .is_some();

    if email_taken || username_taken {
        return HttpResponse::BadRequest().body("Email or username has been taken.");
    }
    // Generate a Zcash wallet for the user
    let (zcash_spending_key, zcash_address) = generate_zcash_wallet();
    // Include the Zcash wallet in the user data using the WalletType enum
    let zcash_wallet = WalletType::Zcash {
        spending_key: zcash_spending_key,
        address: zcash_address,
    };

    let mut user_data = data.into_inner();
    user_data.wallets.push(zcash_wallet);

    let result = users_collection
        .insert_one(user_data, None)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User registered."),
        Err(_) => HttpResponse::InternalServerError().body("Error registering user."),
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub enum WalletType {
    Zcash {
        spending_key: String,
        address: String,
    },
    Bitcoin {
        private_key: String,
        address: String,
    },
    Ethereum {
        private_key: String,
        address: String,
    },
}
impl UserData {
    pub fn new(username: String, email: String, password: String) -> Self {
        UserData {
            username,
            email,
            password,
            wallets: Vec::new(),
            subscriptions: Vec::new(),
            interests: Vec::new(),
            posts: Vec::new(),
            interactions: Vec::new(),
            contracts: Vec::new(),
        }
    }
}