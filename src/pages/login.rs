use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::user::User;

#[component]
pub fn LoginView(action: ServerAction<Login>) -> impl IntoView {
	let _ = action;
	let action = ServerAction::<Login>::new();
    let pending = action.pending();

    let is_err = Signal::derive(move || {
        action.value().get().is_some_and(|result| result.is_err()) && !pending.get()
    });
    let err_string = Signal::derive(move || {
        action
            .value()
            .get()
            .map(|res| match res {
                Ok(_) => String::new(),
                Err(e) => e.to_string(),
            })
            .unwrap_or_default()
    });

    let query = use_query_map();

    view! { // idk how to have this actually autocomplete
    	<ActionForm action=action attr:class="login_form">
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
  			<label class="type_input_label">
   				<input
    				id="password"
    				name="password"
    				type="password"
    				autocomplete="current-password"
    				required
    			/>
    			<span class="input_label">"Password"</span>
    		</label>
			<button class="btn submit_btn" type="submit">"Submit"</button>
	    </ActionForm>
	     <Show when=move || is_err.get()>
             <p>{err_string}</p>
         </Show>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: User,
}

#[tracing::instrument(skip_all)]
#[server(Login, "/api", endpoint = "auth/login")]
pub async fn login(
    name: String,
    password: String,
) -> Result<LoginResponse, ServerFnError> {
	use sqlx::Row;
	use tower_sessions::Session;
	use crate::state::AppState;
	use crate::utils::password::*;

	let pool = expect_context::<AppState>().pool;
	let session = expect_context::<Session>();

	let login_error = ServerFnError::new("Invalid credentials.");

	let get_query = "SELECT * FROM Users WHERE name = ?";
	// let user_res = sqlx::query_as::<_, User>(get_query)
	let user_res = sqlx::query(get_query)
		.bind(&name)
		.fetch_one(&pool)
		.await
		.map_err(|_| login_error.clone())?;

	// do I match this too?
	let is_valid = validate(&password, user_res.get("password"));
	match is_valid {
		Err(_) => return Err(login_error.clone()),
		_ => (),
	};
	

	let user_id: String = user_res.get("id");
	let user = User {
        id: user_id.clone(),
        name: user_res.get("name"),
    };
    
	crate::utils::auth::set_token("token".into(), user_id.clone().into()).await;
	session.insert("user_id", user_id).await.unwrap();
	
	leptos_axum::redirect("/");
	Ok( LoginResponse { user } )
}

#[tracing::instrument(skip_all)]
#[server(Logout, "/api", endpoint = "auth/logout")]
pub async fn logout(/*session: Session*/) -> Result<(), ServerFnError> {
	// session.clear().await;
    Ok(())
}

