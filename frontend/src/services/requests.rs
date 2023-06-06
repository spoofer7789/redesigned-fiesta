use serde_json;
use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Serialize};
use reqwasm::http::{Request, Method, Response};
use crate::error::Error;
use crate::types::ErrorInfo;

 const API_ROOT: &str = "/backend";
 const TOKEN_KEY: &str = "yew.token";

lazy_static! {
    // Jwt token read from local storage.
    pub static ref TOKEN: RwLock<Option<String>> = {
        if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

// Set jwt token to local storage.
pub fn set_token(token: Option<String>) {
    if let Some(t) = token.clone() {
        LocalStorage::set(TOKEN_KEY, t).expect("failed to set");
    } else {
        LocalStorage::delete(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write();
    *token_lock = token;
}

//Get jwt token from lazy static.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}

pub async fn request<B, T>(method: &str, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == "POST" || method == "PUT";
    let url = format!("{}{}", API_ROOT, url);
    let method = match method {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        _ => return Err(Error::UnknownMethod),
    };

    let mut req = Request::new(&url)
        .method(method)
        .header("Content-Type", "application/json");

    if let Some(token) = get_token() {
        req = req.header("Authorization", &format!("Token {}", token));
    }

    if allow_body {
        let body_json = serde_json::to_string(&body).expect("Failed to serialize request body");
        req = req.body(body_json);
    }

    let response = req.send().await;

    match response {
        Ok(data) => {
            if data.status() >= 200 && data.status() < 300 {
                let data: Result<T, _> = serde_json::from_str(data.text().await.unwrap().as_str());
                match data {
                    Ok(data) => {
                        log::debug!("Response: {:?}", data);
                        Ok(data)
                    }
                    Err(_) => Err(Error::DeserializeError),
                }
            } else {
                match data.status() {
                    401 => Err(Error::Unauthorized),
                    403 => Err(Error::Forbidden),
                    404 => Err(Error::NotFound),
                    500 => Err(Error::InternalServerError),
                    422 => {
                        let data: Result<ErrorInfo, _> = serde_json::from_str(data.text().await.unwrap().as_str());
                        match data {
                            Ok(data) => Err(Error::UnprocessableEntity(data)),
                            Err(_) => Err(Error::DeserializeError),
                        }
                    }
                    _ => Err(Error::RequestError),
                }
            }
        },
        Err(_) => Err(Error::RequestError),
    }
}

// Delete request
pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request("DELETE", url, ()).await // <- String literal here
}

// Get request
pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request("GET", url, ()).await // <- String literal here
}

// Post request with a body
pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request("POST", url, body).await // <- String literal here
}

// Put request with a body
pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request("PUT", url, body).await // <- String literal here
}
// Set limit for pagination
pub fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}


// build all kinds of http request: post/get/delete etc.

