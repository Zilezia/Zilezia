use leptos::prelude::*;

#[island]
pub fn ToggleProvider(children: Children) -> impl IntoView {
	provide_context(RwSignal::new(false));
	children()
}

#[island]
pub fn CardToggle(text: String) -> impl IntoView {
	let (card_toggle, set_card_show) = expect_context::<RwSignal<bool>>().split();
	view! {
		<button
			class="btn"
			on:click=move |_| {
				log::info!("pressed toggle");
				set_card_show.update(|n| *n = !*n);
			}
		>
			{text}
		</button>
	}
}

#[island]
pub fn CardShow(is: bool, children: Children) -> impl IntoView {
	let card_toggle = expect_context::<RwSignal<bool>>().read_only();
    view! {
	    <div
	    	style="transition: .5s"
	    	style:opacity=move || if card_toggle.get() == is {"0"} else {"1"}
	    	style:display=move || if card_toggle.get() == is {"contents"} else {"none"}
        >{children()}</div>
    }
}
