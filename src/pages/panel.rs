use leptos::prelude::*;

use crate::components::{
	private::*,
	TitleAndDescription,
};

#[component]
pub fn Panel() -> impl IntoView {
    view! {
    	<TitleAndDescription title="Panel " desc="Panel Description"/>
    	<p>"Sinethung?"</p>
        // what do I put here site traffic maybe?
    }
}
