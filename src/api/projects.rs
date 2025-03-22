use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;

use crate::user::*;
use crate::components::projects::card::*;
use crate::models::*;

#[tracing::instrument(skip_all)]
#[server(ProjectApi, "/api/projects", endpoint = "test")]
pub async fn please_work_api() -> Result<(), ServerFnError> {
    let Some(_) = crate::utils::auth::get_token() else {
    	return Err(ServerFnError::new("Error"));
    };

    Ok(())
}

#[tracing::instrument(skip_all)]
#[server(EditProject, "/api/projects", endpoint = "edit")]
pub async fn edit_project(id: String) -> Result<(), ServerFnError> {
    let Some(_) = crate::utils::auth::get_token() else {
    	return Err(ServerFnError::new("Error"));
    };

    Ok(())
}

#[tracing::instrument(skip_all)]
#[server(TrashProject, "/api/projects", endpoint = "trash")]
pub async fn trash_project(id: String) -> Result<(), ServerFnError> {
    let Some(_) = crate::utils::auth::get_token() else {
    	return Err(ServerFnError::new("Error"));
    };
    
	use leptos::context::use_context;
    use sqlx::FromRow;
    use tower_sessions::Session;
    use crate::state::AppState;
    use crate::models::*;

    let pool = expect_context::<AppState>().pool;
	let delete_query = "DELETE FROM Projects WHERE id = ?";
	let result = sqlx::query(delete_query)
		.bind(&id)
		.execute(&pool)
		.await
		.map_err(|e| ServerFnError::new(e));

    Ok(())
}

#[tracing::instrument(skip_all)]
#[server(AddProject, "/api/projects", endpoint = "add")]
pub async fn add_project(
	name: String,
	title: String,
	project_type: String,
	url: Option<String>,
	repo: Option<String>,
	blog: Option<bool>,
) -> Result<(), ServerFnError> {
    let Some(_) = crate::utils::auth::get_token() else {
    	return Err(ServerFnError::new("API Auth Error"));
    };

	use leptos::context::use_context;
    use sqlx::FromRow;
    use tower_sessions::Session;
    use crate::state::AppState;
    use crate::models::*;

    let pool = expect_context::<AppState>().pool;

	let post_query = "INSERT INTO Projects \
		( name, title, project_type, url, repo, blog ) \
		VALUES ( ?, ?, ?, ?, ?, ? )";
	let result = sqlx::query(post_query)
		.bind(&name)
		.bind(&title)
		.bind(&project_type)
		.bind(&url)
		.bind(&repo)
		.bind(&blog)
		.execute(&pool)
		.await
		.map_err(|e| {
			tracing::error!("Insertion error: {:?}", e);
			ServerFnError::new("Database Error")
		});
    
    Ok(())
}

#[tracing::instrument(skip_all)]
#[server(GetProjects, "/api/projects", endpoint = "get")]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
	use leptos::context::use_context;
    use sqlx::FromRow;
    use tower_sessions::Session;
    use crate::state::AppState;
    use crate::models::*;
    
    let pool = expect_context::<AppState>().pool;
	let get_query = "SELECT * FROM Projects";
	let rows = sqlx::query_as::<_, ProjectRow>(get_query)
		.fetch_all(&pool)
		.await
		.expect("failed to query");

    let projects = rows
        .into_iter()
        .map(|row| row.into_project())
        .collect::<Result<Vec<Project>, _>>()
        .expect("Reason")
		// .map_err(|e| ServerFnError::new(e))?
	;

	Ok(projects)
}
