mod components;
mod routes;
mod views;
mod services;
use yew::prelude::*;
use routes::router::Browserfunc;
use crate::services::auth;
use std::rc::Rc;


#[function_component]
pub fn App() -> Html {
    html! {
      <>
            <Browserfunc />
      </>
    }
}
