use reqwasm::http::Request;
use serde::Deserialize;
use crate::services::auth::get_jwt_token;
use web_sys::{WebSocket, MessageEvent};
use wasm_bindgen::{closure::Closure, JsCast};
use js_sys;
use std::rc::Rc;
use std::cell::RefCell;
use yewdux::prelude::*;
//should work as both a models and resthelper.




pub async fn fetch_user_data(data_type: &str) -> String {
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

    ws.send_with_str(data_type).unwrap();
    rx.await.expect("Failed to receive data")
}

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

