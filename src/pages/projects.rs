// I kinda want this to be like a hub for my live? things, showing the versions status etc // what am  i doing with myself
    	
use std::sync::{ Arc, Mutex };
use std::collections::{ HashMap, HashSet };
use futures::stream::Stream;
use leptos::prelude::*;
use leptos_meta::Title;

use reactive_stores::Store;

use crate::models::*;
use crate::api::projects::*;
use crate::user::{ WriteUser, ReadUser, get_user };
use crate::components::{
	Loading,
	TitleAndDescription,
	toggle::*,
	auth::Auth,
	accordion::*,
	projects::card::*,
};


// for checking if page loaded is an iframe
// const inIframe = window.self !== window.top;

#[component]
pub fn Projects() -> impl IntoView {
	let projects = Resource::new(
		|| (),
		|_| async move {
			get_projects().await
		}
	);

	// TODO: rewrite both these views
	let nav_view = move || {
		projects.with(move |x| {
	        x.clone().map(move |res| {
	            let projects = res.unwrap_or_default();
	            let mut categories = Vec::new();
                for project in projects {
                    if !categories.contains(&project.project_type) {
                        categories.push(project.project_type.clone());
                    }
                }
                categories.sort();
                
	            view! {
	            	<nav class="projects_nav">
		           		<For
		                    each=move || categories.clone().into_iter().enumerate()
		                    key=|(i, _)| *i
		                    children=move |(_, category): (usize, String)| {
		                    	view!{
		                    		<a href=format!("#{category}")>{category.clone()}</a>
		                    	}
		                	}
	                	/>
	            	</nav>
            	}
			})
     	})
	};

	let projects_view = move || {
	    projects.with(move |x| {
	        x.clone().map(move |res| {
	            let projects = res.unwrap_or_default();
	            let mut grouped_projects: HashMap<String, Vec<Project>> = HashMap::new();

	            for project in projects {
	                grouped_projects.entry(project.project_type.clone())
	                    .or_insert_with(Vec::new)
	                    .push(project);
	            }

	            let mut sorted_categories: Vec<(String, Vec<Project>)> = grouped_projects.into_iter().collect();
	            sorted_categories.sort_by(|a, b| a.0.cmp(&b.0));

	            view! {
	                <For
	                    each=move || sorted_categories.clone().into_iter().enumerate()
	                    key=|(i, _)| *i
	                    children=move |(_, (category, projects)): (usize, (String, Vec<Project>))| {
	                        view! {
                           		<AccordionProvider>
                      	        	<AccordionToggle title=category.to_string()/>
                      	            <AccordionShow is=false>
                      	            	<div class="projects_container">
				                            <For
			                                    each=move || projects.clone().into_iter().enumerate()
			                                    key=|(i, _)| *i
			                                    children=move |(_, project): (usize, Project)| {
			                                        let project = RwSignal::new(project);
			                                        view! { <View project/> }
			                                    }
			                                />
                   	    				</div>
                  	            	</AccordionShow>
                  	            </AccordionProvider>
	                        }
	                    }
	                />
	            }
	        })
	    })
	};
	
	view! {
    	<TitleAndDescription
    		title="Projects "
    		desc="Majority of my projects that I find interesting to display on here at least."
    	/>
    	<h1>"Projects"</h1>
    	<Suspense fallback=Loading>
    		{nav_view}
   		</Suspense>

   		// this is stupid /gen
    	<Auth>
       		<ToggleProvider>
       			<Toggle text="+".into() attr:class="btn"/>
       			<ToggleShow is=true>
       				<AddCard attr:class="project_add_card"/>
       			</ToggleShow>
       		</ToggleProvider>
    	</Auth>
    	
    	<section class="projects_section">
    		<div class="section_wrapper">
    			// hmm yes
    			<Suspense fallback=Loading>
    				{projects_view}
   				</Suspense>
    		</div>
    	</section>
    }
}
