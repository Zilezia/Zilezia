mod components;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::{
    home::Home, 
    notfound::NotFound
};
use components::header::Header;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {<div id="root">
        <div class="main_container">
            <Header />
            <div class="main">
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </div>
        </div>
    </div>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}