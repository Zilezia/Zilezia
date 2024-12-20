use crate::store::Store;
use crate::router::{ switch, Route };

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[function_component(App)]
pub fn app() -> Html {
    // all at just have for later
    let (store, _dispatch) = use_store::<Store>();
    let is_page_loading = store.page_loading.clone();

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
            if is_page_loading {}
        </BrowserRouter>
    }
}
