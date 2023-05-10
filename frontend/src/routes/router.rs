use yew::prelude::*;
use yew_router::prelude::*;
use crate::views::pages::home::Home;
use crate::views::pages::{createaccount::CreateAccount, login::Login};
use crate::{routes::search::*,components::medium::web3::Web3Login};
use crate::views::pages::settings::Settings;
use yew::functional::use_context;
use crate::services::header::fetch_user_data;
use super::search::{SearchRoute,switch_search};
#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/web3login")]
    Web3Login,
    #[at("/createaccount")]
    CreateAccount,
    #[at("/{search}")]
    SearchRoot,
    #[at("/search/")]
    Search,
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound
}
#[derive(Clone, Routable, PartialEq)]
enum SettingsRoute {
    #[at("/settings")] //Create an Account for 
    GeneralSettings, 
    #[at("/settings/wallets")]
    Wallets,
    #[at("/settings/mycontracts")]
    MyContracts,
    #[at("/settings/privacy")]
    Privacy,
}


fn switch_main(route: MainRoute) -> Html {
match route {
    MainRoute::Home => html! {<Home/>},
    MainRoute::Login => html! {<Login/>},
    MainRoute::Web3Login => html! {<Web3Login/>},
    MainRoute::CreateAccount => html! {<CreateAccount/>},
    MainRoute::SearchRoot | MainRoute::Search=> html! {<Switch<SearchRoute> render={switch_search}/>},
    MainRoute::SettingsRoot | MainRoute::Settings=> html! {<Switch<SettingsRoute> render={switch_settings}/>},
    MainRoute::NotFound => html!{<>{"404 Not Found!"}</>},
}
} // Settings should only be available for loggedin users.
fn switch_settings(route: SettingsRoute) -> Html {
    match route {
        SettingsRoute::GeneralSettings => html! {<Settings/>},
        SettingsRoute::Wallets => {  html! (format!(""))},
        SettingsRoute::MyContracts => html! (format!("")),
        SettingsRoute::Privacy => html! (format!("")),
    }
}
#[function_component]
pub fn Browserfunc() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={switch_main}/>
        </BrowserRouter>
    }
}