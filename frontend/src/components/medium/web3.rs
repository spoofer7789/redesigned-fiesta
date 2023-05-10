
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_ethereum_provider::{ 
    chain::{self, ethereum}, AccountLabel, EthereumContextProvider, ConnectButton, 
};
use crate::components::small::web3login::Web3Log;
#[function_component]
pub fn Web3Login() -> Html {
   
//let realaddress = ethereum.display_address();
    html! {
        <div>
            <EthereumContextProvider>
                <ConnectButton>
                    <button>{"Connect"}</button> // when the user connects 
                    //their ethereum account it will redirect them to choose a username.
                </ConnectButton>
                <AccountLabel />
                <Web3Log/>
            </EthereumContextProvider>
        
        </div>
    }
}