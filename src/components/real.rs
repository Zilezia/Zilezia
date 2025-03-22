// we making this into a cookie hell yeah who doesn't love cookies 🗣️ 🗣️ 🗣️ 🗣️ 🗣️ 

// ill write this in a moment 

use leptos::prelude::*;

use super::Fur;

#[island]
pub fn RealProvider(children: Children) -> impl IntoView {
	provide_context(RwSignal::new(false));
	children()
}

#[island]
pub fn Real() -> impl IntoView {
	let (card_toggle, set_card_show) = expect_context::<RwSignal<bool>>().split();
	view! {
		<button
			class="btnless"
			on:click=move |_| {
				log::info!("pressed toggle");
				set_card_show.update(|n| *n = !*n);
			}
		>
			<Fur/>
		</button>
	}
}

#[island]
pub fn RealShow(is: bool, children: Children) -> impl IntoView {
	let card_toggle = expect_context::<RwSignal<bool>>().read_only();
    view! {
	    <div
	    	style="transition: .5s"
	    	style:opacity=move || if card_toggle.get() == is {"0"} else {"1"}
	    	style:display=move || if card_toggle.get() == is {"contents"} else {"none"}
        >{children()}</div>
    }
}
