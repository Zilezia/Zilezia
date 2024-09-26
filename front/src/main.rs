mod components;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::{
    well_be_back::WellBBack,
    // home::Home,
    // readme::Readme,
    notfound::NotFound
};
// use pages::*::*;
use components::header::Header;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    WellBBack,
    #[not_found]
    #[at("/404")]
    NotFound
    // #[at("/")]
    // Home,
    // #[at("/readme")]
    // Readme,
    // #[not_found]
    // #[at("/404")]
    // NotFound
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::WellBBack => html! { <WellBBack /> },
        // Route::Home => html! { <Home /> },
        // Route::Readme => html! { <Readme /> },
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