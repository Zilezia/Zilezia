use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;

use super::AuthPage;
use crate::components::{ Header, Cookie };

#[component(transparent)]
pub fn AuthOutlet(cookie_shown: bool) -> impl IntoView {
	view! {
		<AuthPage>
			<PageOutlet cookie_shown/>
		</AuthPage>
   	}
}

#[component(transparent)]
pub fn PageOutlet(cookie_shown: bool) -> impl IntoView {
	view! {
		<PageTemplate cookie_shown>
			<Outlet/>
		</PageTemplate>
   	}
}

#[component]
pub fn PageTemplate(cookie_shown: bool, children: Children) -> impl IntoView {
	view! {
		<Header/>
       	<main id="main_container">
       		{children()}
       	</main>
       	<Show when=move || cookie_shown>
       		<Cookie/>
		</Show>
   	}
}
