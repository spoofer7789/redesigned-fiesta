use crate::services::auth::{*, self};
use crate::routes::router::MainRoute;
use serde_json;
use yew::prelude::*;
use yew_router::prelude::*;
use yew::events::SubmitEvent;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use serde::Serialize;
use reqwasm::http::Request;
use wasm_bindgen_futures::*;
use crate::main::{get_user_context, UserContext};


// Inside login function
#[derive(Debug, Serialize)]
struct LoginData {
    identifier: String,
    password: String,
}

#[function_component(Loginform)]
pub fn login() -> Html {
    let user = use_state(|| "".to_string());
    let pass = use_state(|| "".to_string());
    let error = use_state(|| "".to_string());
    let is_logged_in = use_state(|| false);

    let onsubmit = {
        let user = user.clone();
        let pass = pass.clone();
        let error = error.clone();
        let is_logged_in = is_logged_in.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let user = user.clone();
            let pass = pass.clone();
            let error = error.clone();
            let is_logged_in = is_logged_in.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post("/backend/login")
                    .header("Content-Type", "application/json")
                    .body(JsValue::from_str(
                        &serde_json::to_string(&LoginData {
                            identifier: (*user).to_string(),
                            password: (*pass).to_string(),
                        })
                        .unwrap(),
                    ))
                    .send()
                    .await;

                match resp {
                    Ok(response) => {
                        if response.status() == 200 {
                            let token = response.text().await.unwrap();
                            auth::save_jwt_token(token);
                            let mut user_context = get_user_context();
                            update_user_context(&mut user_context, auth::get_jwt_token(), true); 
                        
                        } else {
                            let response_text = response.text().await.unwrap();
                            error.set(response_text);
                        }
                    }
                    Err(err) => {
                        error.set(format!("Error: {:?}", err));
                    }
                }
            });
        })
    };

                let user_onchange = {
                    let user = user.clone();
                    Callback::from(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        user.set(input.value());
                    })
                };

                let pass_onchange = {
                    let pass = pass.clone();
                    Callback::from(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        pass.set(input.value());
                    })
                };
                let user_context = get_user_context();

                if let Some(user_context) = user_context.as_ref() {
                    if user_context.is_logged_in {
                        return html! {
                            <Redirect<MainRoute> to={MainRoute::Home}/>
                        };
                    }
                }
        html! {
            // Your original form and error handling code here
            <>
            <section class="section">
                <div class="container">
                    <div class="columns is-centered">
                        <div class="column is-four-fifths">
                            <div class="card">
                                <header class="card-header">
                                    <p class="card-header-title">
                                        {"Login"}
                                    </p>
                                </header>
                                <div class="card-content">
                                    <div class="content">
                                        if error.len() > 0 {
                                            <div class="notification is-danger">
                                               {(*error).clone()}
                                            </div>
                                        }
                                        <form {onsubmit}>
                                            <div class="field">
                                                <label class="label">{"Username"}</label>
                                                <div class="control">
                                                    <input onchange={user_onchange} value={(*user).clone()} class="input" type="text" placeholder="username" />
                                                </div>
                                            </div>

                                            <div class="field">
                                                <label class="label">{"Password"}</label>
                                                <div class="control">
                                                    <input onchange={pass_onchange} value={(*pass).clone()} class="input" type="password" placeholder="password" />
                                                </div>
                                            </div>

                                            <div class="field">
                                                <div class="control">
                                                    <button class="button is-link" type="submit">{"Login"}</button>
                                                </div>
                                            </div>
                                        </form>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </>
        }
    }

