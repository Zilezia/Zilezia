use leptos_meta::Title;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::user::*;
use crate::pages::login::Logout;
use crate::components::auth::Auth;
use crate::components::icon::Smull;

#[component]
pub fn Header(
	logout: ServerAction<Logout>,
	set_user: WriteUser,
) -> impl IntoView {
    view! {
		<header id="main_header">
			// make this maybe show on some routes it might be annoying in hindsight (correct word?)
			<Smull/>
			<A href="">"Home"</A>
			<A href="projects">"Projects"</A>

			<A href="blog">"Blog"</A>
			// TODO ion wanna now
			// <div class="header_dropdown">
				// <p class="dropdown_title" title="boring stuff *yawn*">"Legal"</p>
				// <div class="dropdown_links">
					// <A href="privacy-policy">"PvP"</A>
					// <A href="terms-of-service">"ToS"</A>
				// </div>
			// </div>
			
			<Auth set_user>
	        	<A href="/panel" attr:class="total_last_link">"Panel"</A>
			</Auth>
		</header>
    }
}
