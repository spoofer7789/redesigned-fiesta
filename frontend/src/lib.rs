mod components;
mod routes;
mod views;
mod services;
mod user_context;
use crate::components::medium::login::Loginform;
use yew::prelude::*;
use routes::router::Browserfunc;
use crate::services::auth;
use std::rc::Rc;
use crate::services::context::{get_user_context, UserContext};
#[function_component]
pub fn App() -> Html {
  let user_context = use_state(|| get_user_context());
    html! {
      <>
      <UserContextProvider value={(*user_context).clone()}>
            <Browserfunc />
            </UserContextProvider>
      </>
    }
}
// log into github! create fucking gitrepo!