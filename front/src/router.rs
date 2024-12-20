use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    home::HomePage
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    /* later
    #[at("/schizo")]
    SchizoPage,
    #[at("/404")]
    HomePage,
    */
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage/>},
        // Route::SchizoPage => html! {<SchizoPage/>},
    }
}
