use yew::functional::{use_effect_with_deps, use_state, UseEffectHandle, UseStateHandle};
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use crate::services::header::fetch_user_data;

#[derive(Deserialize, Debug, Clone)]
pub struct UserData {
    username: String,
}

#[function_component(Settings)]
pub fn settings() -> Html {
    let (user_data, set_user_data): (UseStateHandle<Option<UserData>>, _) = use_state(|| None);

    use_effect_with_deps(
        move |_: &Option<UserData>| {
            let future = async {
                let username = fetch_user_data("username").await;
                set_user_data(Some(UserData { username }));
            };
            wasm_bindgen_futures::spawn_local(future);
            UseEffectHandle::default()
        },
        (),
    );

    html! {
        <>
            <h1>{"User Settings"}</h1>
            {
                if let Some(user_data) = &*user_data {
                    html! { <h2>{ format!("{:?}", user_data.username) }</h2> }
                } else {
                    html! {}
                }
            }
        </>
    }
}
