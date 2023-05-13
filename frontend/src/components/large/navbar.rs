use yew::prelude::*;
use yew_router::{prelude::use_navigator, navigator};
use crate::routes::{*, router::MainRoute, search::SearchRoute};
use crate::services::header::fetch_user_data;
use std::rc::Rc;
use crate::services::context::UserContext;

#[function_component]
pub fn Navbar() -> Html {
    let user_context = get_user_context();
    let navigator = use_navigator().unwrap();

    let home = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&MainRoute::Home));
        html! {
            <button {onclick}>{"Home"}</button>
        }
    };

    let settings = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&MainRoute::Settings));
        html! {
            <button {onclick}>{"Settings"}</button>
        }
    };

    let login = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&MainRoute::Login));
        html! {
            <button {onclick}>{"Login"}</button>
        }
    };

    let createaccount = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&MainRoute::CreateAccount));
        html! {
            <button {onclick}>{"Create Account"}</button>
        }
    };

    let loggedin_navbar = html! {
        <div id="logged-in-navigation">
            {home}
            <span id="username-display">{"Welcome, "}</span>
            {settings}
        </div>
    };

    let logged_out_navbar = html! {
        <div id="logged-out-navigation">
            {login}
            {createaccount}
        </div>
    };
    let user_context = use_context::<Option<Rc<UserContext>>>().unwrap();
    html!{
    if let Some(user_context) = user_context.as_ref()   {
            {loggedin_navbar}
            } else {
        {logged_out_navbar}
        }
    }
    }
