// use frontend::App;
// use yewdux::prelude::*;

// fn main() {
//     yew::Renderer::<App>::new().render();
//     wasm_logger::init(wasm_logger::Config::default());

// }
use frontend::App;
use yew::prelude::*;
use yewdux::prelude::*;
use wasm_logger::init;
use wasm_logger::Config;
use crate::services::context::UserContext;


fn main() {
    yew::initialize();
    init(Config::default());
    
    let context = UserContext {
        jwt_token: None,
        is_logged_in: false,
    };
    
    let store = BasicStore::new(context);
    let app: App<BasicStore<UserContext>> = App::new(store);
    yew::start_app_in_element::<StoreProvider<BasicStore<UserContext>>>(
        yew::utils::document().get_element_by_id("app").unwrap(),
        app,
    );
}
