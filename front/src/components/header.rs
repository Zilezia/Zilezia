use crate::router::{ self, Route };
use stylist::css;
use yew::prelude::*;
use yew_router::prelude::*;
// I dont really need this
#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
            <nav class="navi">
                <Link<Route> to={Route::HomePage}>{"Home"}</Link<Route>>
            </nav>
        </header>
    }
}
