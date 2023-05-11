use yew::prelude::*;
use serde::{Deserialize, Serialize};
use crate::services::header::fetch_user_data;


#[function_component(Settings)]
pub fn settings() -> Html {
 


    html! {
        <>
            <h1>{"User Settings"}</h1>
            {
                // if let Some(user_data) = &*user_data {
                //     html! { <h2>{ format!("{:?}", user_data.username) }</h2> }
                // } else {
                //     html! {}
                // }
            }
        </>
    }
}
