// use yew::prelude::*;
// use yew_functional::function_component;
// use yew_functional::use_context;
// use std::rc::Rc;

// use crate::components::large::navbar::Navbar;
// use crate::services::header::fetch_user_data;
// use crate::services::auth::get_user_context;
// use crate::services::context::UserContext;

// #[function_component]
// pub fn Profile() -> Html {
//     let context = use_context::<Rc<UserContext>>().expect("Failed to get UserContext");

//     // Fetch user data if not already fetched
//     if context.user_data.is_none() {
//         // fetch_user_data function is async so you need to handle it properly 
//         // here is a simplified example and might not work as it is. 
//         // In real case, you might want to use yew's services or hooks to handle async tasks.
//         // You might also want to handle errors in case the fetch fails.
//         let context_clone = context.clone();
//         let task = yew::services::fetch::FetchService::fetch(
//             Request::get("http://localhost:8000/user_data")
//                 .body(Ok("".to_string()))
//                 .expect("Could not build that request."),
//             Callback::from(move |_| {
//                 let user_data = ...; // parse user_data from response
//                 update_user_context(&mut context_clone, context.jwt_token.clone(), context.is_logged_in, Some(user_data));
//             }),
//         );
//     }
    
//     let user_data = context.user_data.as_ref().map(|data| html! {
//         <>
//             <p>{ &data.username }</p>
//             <p>{ &data.email }</p>
//             // add more fields as per your UserData struct
//         </>
//     });

//     html! {
//         <>
//             <Navbar/>
//             <div>
//                 { "User Profile" }
//                 { user_data.unwrap_or_else(|| html!{ "Loading user data..." }) }
//             </div>
//         </>
//     }
// }
