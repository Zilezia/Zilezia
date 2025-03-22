cfg_if::cfg_if! {
	if #[cfg(feature = "ssr")] {
		use sqlx::MySqlPool;
		use axum::extract::FromRef;
		use tower_sessions::MemoryStore;
		use leptos_axum::AxumRouteListing;
		use leptos::prelude::LeptosOptions;
		
		#[derive(FromRef, Debug, Clone)]
		pub struct AppState {
			pub pool: MySqlPool,
			pub routes: Vec<AxumRouteListing>,
			pub leptos_options: LeptosOptions,
			pub session_store: MemoryStore, // change to SessionStore
		}
	}
}
