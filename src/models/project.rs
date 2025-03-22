use leptos::prelude::*;
use reactive_stores::Store;
use serde::{Serialize, Deserialize};

// using this?
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
	Website,
	Misc,
	Other,
}

#[allow(non_snake_case)]
#[derive(Store, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,				// the id
    pub name: String,			// name
    pub title: String,			// more exagerrative name
    pub project_type: String,	// category of the project (above)
    pub url: Option<String>,	// if it is of website type then shows it off in an iframe
    pub repo: Option<String>,	// if i wanna or i can have the url to github repo
    pub blog: bool,				// did i make a blog tutorial for it
}

pub type ProjectSignal = RwSignal<Project>;

cfg_if::cfg_if! {
	if #[cfg(feature="ssr")] {
		use sqlx::FromRow;

		#[allow(non_snake_case)]
		#[derive(FromRow, Debug, Serialize, Deserialize)]
		pub struct ProjectRow {
			pub id: String,
		    pub name: String,
		    pub title: String,
		    pub project_type: String,
		    pub url: Option<String>,
		    pub repo: Option<String>,
		    pub blog: Option<i8>,
		}

		impl ProjectRow {
		    pub fn into_project(self) -> Result<Project, String> {
		        Ok(Project {
		            id: self.id,
		            name: self.name,
		            title: self.title,
		            project_type: self.project_type,
		            url: self.url,
		            repo: self.repo,
		            blog: self.blog.unwrap_or(0) != 0,
		        })
		    }
		}
	}
}
