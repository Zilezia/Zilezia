cfg_if::cfg_if! {
	if #[cfg(feature = "ssr")] {
		use crate::app::shell;
		use crate::state::AppState;
		use crate::components::error::AppError;

		use axum::{
			body::Body,
			extract::State,
			http::{ Request, Response, StatusCode, Uri },
			response::{ IntoResponse, Response as AxumResponse },
		};

		use leptos::prelude::*;
		use leptos_axum::handle_server_fns_with_context;

		use tower::ServiceExt;
		use tower_sessions::Session;
		use tower_http::services::ServeDir;

		use http::{ HeaderValue, header::CACHE_CONTROL };

		pub async fn file_and_error_handler(
			uri: Uri,
			State(app_state): State<AppState>,
			request: Request<Body>
		) -> AxumResponse {
			let root = &app_state.leptos_options.site_root;
			let mut res = get_static_file(&uri, root).await.unwrap();

			if res.status() == StatusCode::OK {
				if uri.path().starts_with("/pkg/") || uri.path().starts_with("/public/") {
					res.headers_mut().insert(
						CACHE_CONTROL,
						HeaderValue::from_static("public, max-age=31536000, immutable")
					);
				}
				res.into_response()
			} else {
				let handler = {
					let leptos_options = app_state.leptos_options.clone();
					leptos_axum::render_app_to_stream_with_context(
						move || {
							provide_context(app_state.clone());
						},
						move || shell(leptos_options.clone()),
					)
				};

				handler(request).await.into_response()
			}
		}

		// async fn get_static_file(uri: &Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
		async fn get_static_file(uri: &Uri, root: &str) -> Result<Response<Body>, AppError> {
			let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
			match ServeDir::new(root).oneshot(req).await {
				Ok(res) => Ok(res.into_response()),
				Err(e) => Err(AppError::InternalError(e.to_string())),
				// Err(e) => Err((
					// StatusCode::INTERNAL_SERVER_ERROR,
					// format!("Something went wrong: {e}"),
				// )),
			}
		}

		use std::net::SocketAddr;
		use axum::extract::ConnectInfo;
		
		#[tracing::instrument(skip_all)]
		pub async fn leptos_routes_handler(
			state: State<AppState>,
			session: Session,
			request: Request<Body>,
		) -> AxumResponse {
			let handler = {
				let State(app_state) = state.clone();
				let app_state = app_state.clone();
				let leptos_options = app_state.leptos_options.clone();
				let session = session.clone();
				leptos_axum::render_route_with_context(
					app_state.routes.clone(),
					move || {
						provide_context(app_state.clone());
						provide_context(session.clone());
					},
					move || shell(leptos_options.clone()),
				)
			};

			handler(state, request).await.into_response()
		}

		#[tracing::instrument(skip_all)]
		pub async fn server_fn_handler(
			State(app_state): State<AppState>,
			session: Session,
			request: Request<Body>,
		) -> impl IntoResponse {
			handle_server_fns_with_context(
				move || {
					provide_context(app_state.clone());
					provide_context(session.clone());
				},
				request,
			)
			.await
			.into_response()
		}
	}
}
