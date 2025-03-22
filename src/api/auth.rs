use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;

use crate::components::projects::{
	card::*,
	props::*,
};
use crate::user::*;

#[tracing::instrument(skip_all)]
#[server(IsAuth, "/api", endpoint = "auth")]
pub async fn please_work_api() -> Result<bool, ServerFnError> {
	// TODO BETTER AUTH!!!! // its good enough shush man
    let Some(_) = crate::utils::auth::get_token() else {
    	return Err(ServerFnError::new("Error"));
    };

    Ok(true)
}
