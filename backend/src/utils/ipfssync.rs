use actix_web::{HttpResponse, Responder,post,web};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
 pub struct Web3LoginData {
    address: String,
    signature: String,
}

#[post("/web3login")]
pub async fn web3_login(
    data: web::Json<Web3LoginData>,
) -> impl Responder {
    // Forward the login data to the Node.js server
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3001/web3login")
        .json(&*data)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}