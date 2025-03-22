use leptos::prelude::*;

use crate::models::*;
use crate::api::projects::*;
use super::icon::Trash as Icon;

#[component]
pub fn Trash(project: ProjectSignal) -> impl IntoView {
	let action = ServerAction::<TrashProject>::new();
	view! {
		<ActionForm action=action>
			// I found some example doing this but I realised this is very very very terrible
			// just like that, you can just edit the dom with developer tools and change the value
			// ofc if the id is existent then it will delete/act something that has that id
			// otherwise (if the id doesn't exist) then well nothing happens

			// atm idc anymore im too frustrated and this "WORKS" and only I SHOULD have access to everything
			// doing these sort of things so im semi unbothered

			// just idk tokenise this better or hmac it or something
			// also create validators that would tie to temporary cookie sessions to make sure this exact
			// project is selected for something (deletion in here)
			<input type="hidden" name="id" value=move || project.with(|x| x.id.to_string()) />
			<button type="submit">
				<Icon/>
			</button>
		</ActionForm>
	}
}
