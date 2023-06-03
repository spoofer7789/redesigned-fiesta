use crate::hooks::use_ethereum::{UseEthereumHandle, UserData};
use yew::prelude::*;
use reqwasm::http::{Request, Response};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::hooks;

pub async fn send_post_request(ethereum: &UseEthereumHandle) -> Result<String, String> {
    if let Some(address) = ethereum.address() {
        let user_data = UserData {
            address: address.to_string(),
            signature: "".to_string(), // Add signature handling if required
        };
        let request_result = Request::post("/web3/web3login")
            .header("Content-Type", "application/json")
            .body(JsValue::from_serde(&user_data).unwrap());
   
    }
    Err("No Ethereum address found".to_string())
}


#[function_component]
pub fn Web3Log() -> Html {
    let ethereum = use_context::<UseEthereumHandle>().expect(
        "no ethereum provider found. you must wrap your components in an <EthereumProvider/>",
    );
    let response_data = use_state(|| String::new());
    let error_message = use_state(|| String::new());

    if ethereum.connected() {
        let ethereum_clone = ethereum.clone();
        let response_data_clone = response_data.clone();
        let error_message_clone = error_message.clone();

        spawn_local(async move {
            match send_post_request(&ethereum_clone).await {
                Ok(data) => response_data_clone.set(data),
                Err(err) => error_message_clone.set(err),
            }
        });
    }

    html! {
        <>
            { if !response_data.is_empty() {
                html! { <div>{format!("Response data: {}", response_data.as_ref() as &str)}</div> }
              } else {
                html! {}
              }
            }
            { if !error_message.is_empty() {
                html! { <div>{format!("Error: {}", error_message.as_ref() as &str)}</div> }
              } else {
                html! {}
              }
            }
        </>
    }
}