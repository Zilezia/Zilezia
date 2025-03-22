use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::TitleAndDescription;
use crate::api::projects::*;

#[component]
pub fn Test() -> impl IntoView {
	let test_project = ServerAction::<ProjectApi>::new();
    view! {
    	<TitleAndDescription title="Test " desc="Debug page of zilezia.dev"/>
    	<ActionForm action=test_project>
	   		<button class="btn">"Test Log"</button>
    	</ActionForm>

    }
}
