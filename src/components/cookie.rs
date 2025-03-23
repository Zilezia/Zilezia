use leptos::prelude::*;
use leptos_router::components::A;

use super::toggle::*;
use super::icon::Cookie as Icon;

#[component]
pub fn Cookie() -> impl IntoView {

	// some toggle maybe quickly explaining cookie usage

    // i kinda wanna do something cool (visually) with the icon i'll have to write my own cookie tho
    view! {
    	<div class="cookie_container">
    		<ToggleProvider>
       			<CookieToggle/>
       			<ToggleShow is=true>
       				<div class="cookie_info">
       					<div class="btn_container">
       						<Toggle text="x".into() attr:class="btn close_btn"/>
       					</div>
       					<h2>"Cookie notice"</h2>
       					<p>"I only use one cookie for person authentication to make sure you're not a bot."</p>
       					<p>"The cookie for that sets when you press when you saw the dog pop up."</p>
       					<p>
       						"More about how I use cookies can be found at "
       						<A href="privacy-policy">"/privacy-policy"</A>
     					</p>
   					</div>
       			</ToggleShow>
       		</ToggleProvider>
    	</div>
    }
}

#[island]
pub fn CookieToggle() -> impl IntoView {
	let (_, set_toggle_show) = expect_context::<RwSignal<bool>>().split();
	
	view! {
		<div
			class="cookie_icon"
			on:click=move |_| { set_toggle_show.update(|n| *n = !*n); }
		><Icon/></div>
	}
}
