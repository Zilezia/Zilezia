use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::TitleAndDescription;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
    	// this doesnt work
    	<TitleAndDescription title="404 " desc=""/>
    	<h1>"Not Found"</h1>
    }
}
