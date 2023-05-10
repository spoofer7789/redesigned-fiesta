use yew_router::prelude::*;
use yew::prelude::*;
#[derive(Clone, Routable, PartialEq)]
pub enum SearchRoute {
    #[at("/search")]
    Search,
   #[at("/search/g/{query}")]  //groups
   G { query: String},
   #[at("/search/u/{query}")] //users
   U { query: String},
   #[at("/search/c/{query}")] //categories
   C { query: String},
   #[at("/search/hashtag/{query}")]
   Hashtag { query: String },
   #[at("/search/cashtag/{query}")]
   Cashtag { query: String },
}

pub fn switch_search(route: SearchRoute) -> Html {
    match route {
        SearchRoute::Search => html! {},
        SearchRoute::G {query} => html! {},
        SearchRoute::U {query} => html! {},
        SearchRoute::C {query: String} => html! {},
        SearchRoute::Hashtag { query } => html! {<p>{format!("Searching with hashtag: #{}", query)}</p>},
        SearchRoute::Cashtag { query } => html! {<p>{format!("Searching with cashtag: ${}", query)}</p>},
    }
}