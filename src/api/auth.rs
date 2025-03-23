use leptos::prelude::*;
use leptos_router::*;

use crate::user::*;
use crate::components::projects::{
	card::*,
	props::*,
};

#[tracing::instrument(skip_all)]
#[server(SetAuthApi, "/api/auth", endpoint = "set")]
pub async fn set_auth_api() -> Result<(), ServerFnError> {
    if let Some(_) = crate::utils::auth::get_token("person_token".into()) {
    	leptos_axum::redirect("/");
    	return Ok(());
    } else {
    	let uid = uuid::Uuid::new_v4().to_string();
    	crate::utils::auth::set_token("person_token".into(), uid).await;
    	
    	leptos_axum::redirect("/");
    	return Ok(());
    };
}

#[tracing::instrument(skip_all)]
#[server(CheckAuthApi, "/api/auth", endpoint = "check")]
pub async fn check_auth_api(current_route: String) -> Result<(), ServerFnError> {
    if let Some(_) = crate::utils::auth::get_token("person_token".into()) {
    	// return Err(ServerFnError::new("Right error"));
    	if current_route == "/auth" {
    		leptos_axum::redirect("/");
   		}
    	return Ok(());
    } else {
    	if current_route != "/auth" {
    		leptos_axum::redirect("/auth");
    	}
    	return Ok(());
    }
}
