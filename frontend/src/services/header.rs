use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use crate::services::auth::get_jwt_token;
use web_sys::{WebSocket, MessageEvent, Document};
use wasm_bindgen::{closure::Closure, JsCast};
use js_sys;
use std::rc::Rc;
use std::cell::RefCell;
use yewdux::prelude::*;

#[derive(Deserialize, Debug)]
pub struct UserData {
    email: String,
    username: String,
    password: String,
    wallets: Vec<WalletType>,
    posts: Vec<Post>,
    subscriptions: Vec<String>,
    interests: Vec<String>,
    interactions: Vec<String>,
    contracts: Vec<WalletType>,
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

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    pub title: String,
    pub body: String,
    pub comments: Vec<Comment>,
    pub file: FileData,  // Assume FileData is a struct that contains file info
}
#[derive(Deserialize, Debug, Clone)]
pub struct FileData {
    pub id: i32,
    pub name: String,
    pub data: String,  // Base64-encoded file data
    // Add more fields as needed
}
pub struct Comment {
    pub username: String,
    pub text: String,
}
pub enum Action {
    SetUserData(UserData),
}

impl Reducer<UserData> for UserData {
    type Action = Action;

    fn new() -> Self {
        Self {
            email: String(),
            username: String::new(),
            password: String::new(),
            wallets: String::new(),
            posts: String::new(),
            interests: String::new(),
            subscriptions: String::new(),
            interactions: String::new(),
            contracts: String::new(),
            // Initialize other fields...
        }
    }

    fn reduce(&mut self, action: Self::Action) -> ShouldRender {
        match action {
            Action::SetUserData(data) => {
                *self = data;
                true
            }
        }
    }
}


pub type UserDataStore = Box<dyn Store>;



pub async fn fetch_user_data() -> Result<UserData, JsValue> {
    let ws = WebSocket::new("ws://localhost:3000/backend/ws/").unwrap();
    let (tx, rx) = futures::channel::oneshot::channel::<String>();
    let tx = Rc::new(RefCell::new(Some(tx)));

    let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        if let Ok(data) = event.data().dyn_into::<js_sys::JsString>() {
            let response = data.as_string().unwrap();
            if let Some(tx) = tx.borrow_mut().take() {
                tx.send(response).expect("Failed to send data");
            }
        }
    }) as Box<dyn FnMut(MessageEvent)>);

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    ws.send_with_str("username").unwrap();
    let response = rx.await.expect("Failed to receive data");

    // Parse the response as a UserData object.
    let user_data: UserData = serde_json::from_str(&response)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse user data: {}", e)))?;

    Ok(user_data)
}
//should work as both a models and resthelper.
//      popular/trending//      take username    
//      wallets
//          wallet type should take in the amounts and addresses.
//              Ring CT
//                  
//              Standard
//                  BTC Doge
//              smart contractbased.
//      contracts
//      notifications    
//          interactions       
//          liked
//          following you
//          commented.
//          request dm
//          bought / sent.
//     feeds, 
//      interests.
//      subscriptions