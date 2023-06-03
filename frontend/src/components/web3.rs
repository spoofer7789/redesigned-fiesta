use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use super::{web3login::Web3Log, account_label::AccountLabel, ethereum_context_provider::EthereumContextProvider, connect_button::ConnectButton};


    use crate::chain::{self, ethereum};


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
               // <AccountLabel />
                <Web3Log/>
         </EthereumContextProvider>

        
        </div>
    }
}