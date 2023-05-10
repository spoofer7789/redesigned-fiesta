mod components;
mod routes;
mod views;
mod services;
mod user_context;
use crate::components::medium::login::Loginform;
use yew::prelude::*;
use routes::router::Browserfunc;
use crate::services::auth;
use yew::context::ContextProvider;
#[function_component]
pub fn App() -> Html {
    html! {
      <>
            <Browserfunc />
      </>
    }
}
// log into github! create fucking gitrepo!