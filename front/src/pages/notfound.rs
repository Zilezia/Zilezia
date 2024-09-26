// jus temp
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::pages::well_be_back::WellBBack;

#[function_component(NotFound)]
pub fn notfound() -> Html {
    // let navigator = use_navigator().unwrap();

    // let go_home_page_button = {
    //     let navigator = navigator.clone();
    //     let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    //     html! {
    //         <button type="button" onclick={onclick} >{ "Home" }</button>
    //     }
    // };

    html! {
      <>
        // <h1>{ "There's no page like that idk go back." }</h1>
        <WellBBack />
      </>
    }
}