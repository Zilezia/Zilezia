use leptos::prelude::*;

#[component]
pub fn At() -> impl IntoView {
	view! {
		<svg xmlns="http://www.w3.org/2000/svg" width="2em" height="2em" viewBox="0 0 24 24">
			<g fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2">
				<path d="M8 12a4 4 0 1 0 8 0a4 4 0 1 0-8 0" />
				<path d="M16 12v1.5a2.5 2.5 0 0 0 5 0V12a9 9 0 1 0-5.5 8.28" />
			</g>
		</svg>
	}
}
