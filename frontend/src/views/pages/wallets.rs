// use yew::prelude::*;
// use reqwasm::http::Request;
// use serde::Deserialize;



// #[derive(Deserialize, Debug)]
// struct UserData {
//     username: String,
//     email: String,
//     password: String,
//     wallets: Vec<Wallet>,
// }

// #[function_component(Model)]
// pub fn model() -> Html {
//     let (user_data, set_user_data) = use_state(|| None::<UserData>);

//     use_effect_with_deps(
//         move |_| {
//             let request = Request::get("/wallets")
//                 .header("Content-Type", "application/json")
//                 .build()
//                 .unwrap();

//             let future = async {
//                 match request.send().await {
//                     Ok(response) => {
//                         if response.status() == 200 {
//                             match response.json::<UserData>().await {
//                                 Ok(user_data) => set_user_data(Some(user_data)),
//                                 Err(_) => set_user_data(None),
//                             }
//                         }
//                     }
//                     Err(_) => set_user_data(None),
//                 }
//             };

//             wasm_bindgen_futures::spawn_local(future);
//             || ()
//         },
//         (),
//     );
//     html! {
//         <>
//             {
//                 if let Some(user_data) = &*user_data {
//                     html! {
//                         // Render the wallets here.
//                         // For example:
//                         for user_data.wallets.iter().map(|wallet| html! { <p>{format!("{:?}", wallet)}</p> })
//                     }
//                 } else {
//                     html! { <p>{ "Loading wallets..." }</p> }
//                 }
//             }
//         </>
//     }
// }
