// TODO:
//   switch to 3D site the "hub" being psx lains menu ¯\_(ツ)_/¯ based icl
//   

mod components;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::{
    statuses::Statuses,
    readme::Readme,
    raw_view::RawView,
    notfound::NotFound,
};
use components::header::Header;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Statuses,
    #[at("/README.md")] // change readme to ./ ?
    Readme,
    #[at("/raw_view")]
    RawView,
    #[not_found]
    #[at("/404")]
    NotFound
    // #[at("/")]
    // WellBBack, // chnage wbb and replace notfound 
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Statuses => html! { <Statuses /> },
        Route::Readme => html! { <Readme /> },
        Route::RawView => html! { <RawView /> },
        Route::NotFound => html! { <NotFound /> },
        // Route::WellBBack => html! { <WellBBack /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {<div id="root">
        <Header />
        <main class="main">
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </main>
    </div>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}