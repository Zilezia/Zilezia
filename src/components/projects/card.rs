use leptos::prelude::*;
use leptos_meta::Style;
use leptos_router::components::*;
use leptos::reactive::wrappers::write::SignalSetter;

use crate::user::*;
use crate::models::*;
use crate::api::projects::AddProject;
use crate::components::{
	auth::Auth,
	icon::Octo,
	edit::Edit,
	trash::Trash,
};

// actual shown thing to everyone (some) kinda TODO
#[component]
pub fn View(project: ProjectSignal) -> impl IntoView {
	// this works?
	let pid = project.with(|x| x.id.clone());
	let pname = project.with(|x| x.name.clone());
	let ptitle = project.with(|x| x.title.clone());
	let ptype = project.with(|x| x.project_type.clone());
	let purl = project.with(|x| x.url.clone());
	let prepo = project.with(|x| x.repo.clone());
	let pblog = project.with(|x| x.blog.clone());

	let set_user = expect_context::<SignalSetter<Option<User>>>();
	
	view! {
		// these shows really carrying
		<Suspense fallback=|| "Loading project...">
			<div class="project_card" id=pid.clone()>
	            <Show
	                when=move || project.with(|x| x.url.clone()).is_some()
	                fallback=|| view! {
	                	// TODO?
	                	<div class="temp">
	                		"no url"
	                	</div>
	                }
                >
                	<a href=purl.clone().unwrap_or_default() title=ptitle.clone()>
		            	<div class="iframe_wrapper">
							<iframe
								class="iframe_project"
								title=ptitle.clone()
								src=purl.clone().unwrap_or_default()
							>"Your browser does not support iframes."</iframe>
						</div>
					</a>
	            </Show>
				<div class="project_info">
					<div class="top_info">
						<p class="project_title">{pname.clone()}</p>
						// idk how to do this tbh
						<Show when=move || pblog>
							<A href="/blog/?">"tut"</A>
						</Show>
					</div>
					<div class="bottom_info">
						<Show when=move || project.with(|x| x.repo.clone()).is_some()>
							<a class="git_icon" href=prepo.clone().unwrap_or_default()>
								<Octo/>
							</a>
						</Show>
					</div>
				</div>
				// this is for me owly grrr
				<Auth>
		       		<div class="edit_strip">
		       			<Edit project/>
		       			<Trash project/>
		       		</div>
		    	</Auth>
			</div>
        </Suspense>
	}
}

// this is for myself less important TODO
#[component]
pub fn AddCard() -> impl IntoView {
	let action = ServerAction::<AddProject>::new();
	view! {
		// <div class="dummy_card_class"> //fr
		<div>
			<h2>"Add project"</h2>
			
	    	<ActionForm action=action attr:class="login_form">

	    		<label class="type_input_label">
	    			<input
	    				id="name"
	    				name="name"
	    				type="text"
	    				required
	    			/>
	    			<span class="input_label">"Project name:"</span>
	    		</label>

	    		<label class="type_input_label">
	    			<input
	    				id="title"
	    				name="title"
	    				type="text"
	    				required
	    			/>
	    			<span class="input_label">"Project title:"</span>
	    		</label>

				// maybe have a dropwdown later
	    		<label class="type_input_label">
	    			<input
	    				id="project_type"
	    				name="project_type"
	    				type="text"
	    				required
	    			/>
	    			<span class="input_label">"Project type:"</span>
	    		</label>

				// this should be dependant on project_type
	    		<label class="type_input_label">
	    			<input
	    				id="url"
	    				name="url"
	    				type="text"
	    			/>
	    			<span class="input_label">"Url:"</span>
	    		</label>

	    		<label class="type_input_label">
	    			<input
	    				id="repo"
	    				name="repo"
	    				type="text"
	    			/>
	    			<span class="input_label">"Repo url:"</span>
	    		</label>

	    		<label class="type_input_label">
	    			<input
	    				id="blog"
	    				name="blog"
	    				type="checkbox"
	    			/>
	    			<span class="input_label">"Blog?"</span>
	    		</label>
	    		
				<button class="btn submit_btn" type="submit">
					"Submit"
				</button>
			</ActionForm>
			
		</div>
	}
}
