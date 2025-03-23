use leptos::prelude::*;

#[island]
pub fn ToggleProvider(children: Children) -> impl IntoView {
	provide_context(RwSignal::new(false));
	children()
}

#[island]
pub fn Toggle(text: String) -> impl IntoView {
	let (_, set_toggle_show) = expect_context::<RwSignal<bool>>().split();
	view! {
		<button on:click=move |_| { set_toggle_show.update(|n| *n = !*n); }>{text}</button>
	}
}

#[island]
pub fn ToggleShow(is: bool, children: Children) -> impl IntoView {
	let toggle_show = expect_context::<RwSignal<bool>>().read_only();
    view! {
	    <div
	    	style="transition: .5s"
	    	style:opacity=move || if toggle_show.get() == is {"0"} else {"1"}
	    	style:display=move || if toggle_show.get() == is {"contents"} else {"none"}
        >{children()}</div>
    }
}
