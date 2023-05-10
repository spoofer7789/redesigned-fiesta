
use yew::prelude::*;
use yewdux::prelude::*;
use crate::state::{Store, Action};
use crate::services::header::fetch_user_data;

#[function_component(Settings)]
pub fn settings() -> Html {
    let dispatch: Dispatch<Store> = use_dispatch();

    use_effect_with_deps(
        move |_| {
            let dispatch = dispatch.clone();
            let future = async move {
                let username = fetch_user_data("username").await;
                dispatch.send(Action::SetUserData(UserData { username }));
            };
            wasm_bindgen_futures::spawn_local(future);
            || ()
        },
        (),
    );

    let user_data = use_state(|| dispatch.state().user_data.clone());

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
