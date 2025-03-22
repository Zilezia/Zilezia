use leptos::prelude::*;

#[component]
pub fn FormInput() -> impl IntoView {
	
    view! {
	    <label class="type_input_label">
   			<input
   				id="name"
   				name="name"
   				type="text"
   				autocomplete="username"
   				required
   			/>
   			<span class="input_label">"Admin"</span>
   		</label>
    }
}
