cfg_if::cfg_if! {
	if #[cfg(feature = "ssr")] {
		use std::env::var;
		use sqlx::MySqlPool;
		use crate::components::error::AppError;

		pub async fn connect() -> Result<MySqlPool, AppError> {
			// really don't need these variables like that I wanted these for a moment for debugging
			let db_user = var("DB_USER").expect("DB_USER not set");
			let db_pass = var("DB_PASSWORD").expect("DB_PASSWORD not set");
			let db_addr = var("DB_ADDR").expect("DB_ADDR not set");
			let db_port = var("DB_PORT").expect("DB_PORT not set");
			let db_name = var("DB_NAME").expect("DB_NAME not set");
			let db_url = format!(
				"mysql://{}:{}@{}:{}/{}",
				db_user, db_pass, db_addr, db_port, db_name,
			);
			match MySqlPool::connect(&db_url).await {
				Ok(pool) => Ok(pool),
				Err(e) => Err(AppError::DatabaseError(e.to_string()))
			}
		}
	}
}
