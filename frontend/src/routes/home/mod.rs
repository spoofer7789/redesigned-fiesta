mod banner;
mod main_view;


use yew::prelude::*;

use banner::Banner;




#[function_component(Home)]
pub fn home() -> Html {
   

    html! {
        <div class="home-page">
            <Banner />
            <div class="container page">
                <div class="row">
                    <div class="col-md-3 col-xs-12">
                        <div class="sidebar">
                          
                           
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
