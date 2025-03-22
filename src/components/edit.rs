use leptos::prelude::*;

use crate::models::*;
use crate::api::projects::*;
use super::icon::Edit as Icon;

#[component]
pub fn Edit(project: ProjectSignal) -> impl IntoView {
	let action = ServerAction::<EditProject>::new();
	view! {
		<ActionForm action=action>
			<input type="hidden" name="id" value=move || project.with(|x| x.id.to_string()) />
			<button type="submit">
				<Icon/>
			</button>
		</ActionForm>
	}
}
