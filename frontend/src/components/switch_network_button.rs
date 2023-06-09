use crate::hooks::use_ethereum::UseEthereumHandle;
use wasm_bindgen_futures::spawn_local;
use web3::transports::eip_1193::Chain;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub chain: Chain,

    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component]
pub fn SwitchNetworkButton(props: &Props) -> Html {
    let ethereum = use_context::<UseEthereumHandle>().expect(
        "no ethereum ethereum found. you must wrap your components in an <Ethereumethereum/>",
    );

    let chain = props.chain.clone();

    let on_click = {
        Callback::from(move |_| {
            let ethereum = ethereum.clone();
            let chain = chain.clone();
            spawn_local(async move {
                //let _ = ethereum.switch_chain_with_fallback(chain).await;
                let _ = ethereum.switch_chain_with_fallback(&chain).await;
            });
        })
    };

    html! {
        <div>
            <button onclick={on_click} class={&props.class}>
                {"Switch to "}{&props.chain.chain_name}
            </button>
        </div>
    }
}
