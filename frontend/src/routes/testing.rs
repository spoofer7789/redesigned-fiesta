use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use crate::components::web3::Web3Login;

#[function_component]
pub fn Testpage() -> Html {
    html! {
        <>
        <Web3Login/>
    </>
    }
}