use web_sys::{Storage, window};
use reqwasm::http::Request;
use serde::Deserialize;
use std::rc::Rc;
use yew::prelude::*;
pub const JWT_STORAGE_KEY: &str = "jwt_token";
pub fn save_jwt_token(token: String) {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(JWT_STORAGE_KEY, &token);
            //Set UserContext to true.
        }
    }
}
pub fn get_jwt_token() -> Option<String> {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            return storage.get_item(JWT_STORAGE_KEY).ok().flatten();
        }
    }
    None
}

