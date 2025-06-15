cfg_if::cfg_if! {
	if #[cfg(feature = "ssr")] {
	
		use log::{Log, Level, Metadata, Record};
		use sqlx::MySqlPool;
		use std::sync::Arc;
		
		// pub struct SqlLogger {
		    // pool: Arc<MySqlPool>,
		// }
		// 
		// impl SqlLogger {
		    // pub fn new(pool: Arc<MySqlPool>) -> Self {
		        // Self { pool }
		    // }
		// }
		// 
		// impl Log for SqlLogger {
		    // fn enabled(&self, metadata: &Metadata) -> bool {
		        // metadata.level() <= Level::Info
		    // }
		// 
		    // fn log(&self, record: &Record) {
		        // if self.enabled(record.metadata()) {
		            // let pool = self.pool.clone();
		            // let level = record.level().to_string();
		            // let target = record.target().to_string();
		            // let message = record.args().to_string();
		// 
	                // tokio::spawn(async move {
    		            // let insert_query = "INSERT INTO test_table (title, completed) VALUES (?, ?)";
           		        // let user_res = sqlx::query(insert_query)
           	        		// .bind(message)
           	        		// .bind(0)
           	        		// .execute(&*pool)
           	        		// .await
           	        		// .unwrap();
		            // });
		        // }
		    // }
		// 
		    // fn flush(&self) {}
		// }

		// pub struct SqlLogger {
		    // pool: Arc<PgPool>,
		// }
		// 
		// impl SqlLogger {
		    // pub fn new(pool: Arc<PgPool>) -> Self {
		        // Self { pool }
		    // }
		// }
		// 
		// impl<S> Layer<S> for SqlLogger
		// where
		    // S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
		// {
		    // fn on_event(
		        // &self,
		        // event: &tracing::Event<'_>,
		        // _ctx: Context<'_, S>,
		    // ) {
		        // let metadata = event.metadata();
		        // let level = metadata.level();
		        // let target = metadata.target();
		        // let mut message = String::new();
		// 
		        // let mut visitor = LogVisitor(&mut message);
		        // event.record(&mut visitor);
		// 
		        // let pool = self.pool.clone();
		        // tokio::spawn(async move {
		            // let _ = sqlx::query!(
		                // r#"
		                // INSERT INTO logs (level, target, message, timestamp)
		                // VALUES ($1, $2, $3, NOW())
		                // "#,
		                // level.to_string(),
		                // target,
		                // message
		            // )
		            // .execute(&*pool)
		            // .await;
		        // });
		    // }
		// }
	}
}
