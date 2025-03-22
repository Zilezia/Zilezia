use leptos::prelude::*;

#[island]
pub fn AccordionProvider(children: Children) -> impl IntoView {
	provide_context(RwSignal::new(false));
	children()
}

#[island]
pub fn AccordionToggle(title: String) -> impl IntoView {
	let (accordion_open, set_accordion_open) = expect_context::<RwSignal<bool>>().split();
	view! {
		<h2 id=title on:click=move |_| set_accordion_open.update(|n| *n = !*n)>
			{title.clone()}
		</h2>
	}
}

#[island]
pub fn AccordionShow(is: bool, children: Children) -> impl IntoView {
	let accordion_open = expect_context::<RwSignal<bool>>().read_only();
    view! {
    	// TODO calculate element height
	    <div 
	    	style:display=move || if accordion_open.get() == is {"contents"} else {"none"}
        >{children()}</div>
    }
}
