use serde::{Serialize,Deserialize};
use leptos::prelude::*;
use leptos::reactive::wrappers::write::SignalSetter;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
	pub id: String,
	pub name: String,
}

pub type ReadUser = Signal<Option<User>>;
pub type WriteUser = SignalSetter<Option<User>>;

// I could maybe make a handler for fetching the appriopriate context?
#[tracing::instrument(skip_all)]
#[server(UserApi, "/api", endpoint = "user")]
pub async fn get_user() -> Result<User, ServerFnError> {
	use leptos::context::use_context;
    use sqlx::Row;
    use tower_sessions::Session;
    use crate::state::AppState;
    
    let pool = use_context::<AppState>()
    	.map(|state| state.pool)
    	.ok_or_else(|| ServerFnError::new("AppState not found"))?;
    let session = use_context::<Session>()
    	.ok_or_else(|| ServerFnError::new("Session not found"))?;
    let user_id = session.get::<String>("user_id").await
    	.map_err(|_| ServerFnError::new("Failed to get user_id from session"))?;
	if let Some(id) = user_id {
		let get_query = "SELECT id, name FROM Users WHERE id = ?";
		// let user_res = sqlx::query_as::<_, User>(get_query)
		let user_res = sqlx::query(get_query)
			.bind(id)
			.fetch_one(&pool)
			.await
			.expect("failed to query"); // error handle
			
		let user = User {
	        // id: user_res.id,
	        // name: user_res.name,
	        id: user_res.get("id"),
	        name: user_res.get("name"),
	    };
	   
	   	Ok(user)
	} else {
		Err(ServerFnError::new("Invalid Session ID"))
	}
}
