// this thing is very messed up I'm using to cookies used for auth,
// actual jwt cookies to auth dangerous api actions
// and the `tower_sessions Session store` cookie for auth routes and elements
// this is like that because I literally first found some examples using the stores
// and then properly found jwt appriopriatelly used so I took that
// no issue is done really just more security + heavy obfuscation I'm starting to not have idea what this code is


use std::env::var;
use std::net::{ Ipv4Addr, Ipv6Addr, SocketAddr };
use dotenv::dotenv;
use axum::{
	Router,
	body::Body,
	extract::ConnectInfo,
	routing::{ get, post },
	response::{IntoResponse, Response},
	middleware::{self, Next},
	http::{ StatusCode, Request },
};
use axum_server::tls_openssl::OpenSSLConfig;

use leptos::prelude::*;
use leptos_axum::{
	LeptosRoutes,
	generate_route_list,
};

use tower::ServiceBuilder;
use tower_http::{
	trace::{self, TraceLayer },
	catch_panic::CatchPanicLayer,
	compression::CompressionLayer,
};

use zilezia_dev::{
	app::App,
	db::connect,
	state::AppState,
	handler::{
		file_and_error_handler,
		leptos_routes_handler,
		server_fn_handler,
		// ip_handler,
	},
};

use tower_sessions::{
	// swap these stores later
	MemoryStore,
	// SessionStore,
	// cookie::SameSite,
	SessionManagerLayer,
};

use tracing::{info, Level};

use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// async fn add_remote_addr_to_extensions(
    // ConnectInfo(addr): ConnectInfo<SocketAddr>,
    // request: Request<Body>,
    // next: Next,
// ) -> Response {
    // let mut request = request;
    // request.extensions_mut().insert(addr);
    // next.run(request).await
// }

async fn add_remote_addr_to_extensions(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let mut request = request;
    let req_format = format!("{:?}", request);
	// log::info!("{:?}", request);
    // Get the headers from the request
    let headers = request.headers();

    // Try to get the original client IP from the X-Forwarded-For header
    let remote_addr = headers
        .get("X-Forwarded-For")
        .and_then(|header_value| header_value.to_str().ok())
        .and_then(|header_str| header_str.split(',').next())
        .and_then(|ip_str| ip_str.trim().parse::<SocketAddr>().ok())
        .unwrap_or(addr); // Fallback to the direct connection address if the header is not present

    // Insert the remote address into the request extensions
    request.extensions_mut().insert(remote_addr);

	let pool = connect().await.expect("Unable to make a pool connection with database");

	let get_query = "INSERT INTO test_table (title, completed) VALUES (?, ?)";
	let user_res = sqlx::query(get_query)
		.bind(&req_format)
		.bind(0)
		.execute(&pool)
		.await
		.unwrap();

    next.run(request).await
}

use tower_http::services::ServeDir;
use tower::util::ServiceExt;
use axum::extract::Host;
use axum::handler::HandlerWithoutStateExt;

#[tokio::main]
async fn main() {
	// simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");
	// env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
	tracing_subscriber::fmt()
       .with_target(true)
       .compact()
       .init();
   	// tracing_subscriber::registry()
   		// .with(fmt::layer().compact().with_target(true))
   		// .with(EnvFilter::from_default_env())
   		// .init();
	    
	dotenv().ok();
    let ssl_path = var("ssl").expect("ssl/ path is not set.");
	let toml = var("toml").expect("Cargo.toml path is not set.");
		
    let conf = get_configuration(Some(&toml)).unwrap();
    // let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;

	let pool = connect().await.expect("Unable to make a pool connection with database");

    let routes = generate_route_list(App);

	// let key = Key::generate();
	// let record = Record {
		// 
	// }
	// let session_store = SessionStore::create();
	let session_store = MemoryStore::default();
	let session_layer = SessionManagerLayer::new(session_store.clone())
		.with_name("session_store") // change that later
		// .with_signed(key) // or with_private maybe
	;
   
	// these clones are just incase if the compiler doesnt like that
	let app_state = AppState {
		pool: pool.clone(),
		routes: routes.clone(),
		leptos_options: leptos_options.clone(),
		session_store: session_store.clone(),
	};

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            let remote_addr = request
                .extensions()
                .get::<SocketAddr>()
                .map(|addr| addr.to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let user_agent = request
                .headers()
                .get("User-Agent")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown");

            tracing::span!(
                Level::INFO,
                "request",
                remote_addr = remote_addr,
                user_agent = user_agent,
                method = %request.method(),
                uri = %request.uri(),
            )
        })
        .on_request(|_request: &Request<_>, _span: &tracing::Span| {
            tracing::info!("started processing request");
        })
        .on_response(|response: &Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
            tracing::info!(
                "finished processing request (latency: {:?}, status: {})",
                latency,
                response.status()
            );
           	let res_format = format!("{:?}", response);
            let get_query = "INSERT INTO test_table (title, completed) VALUES (?, ?)";
            tokio::spawn(async move {
	           	let user_res = sqlx::query(get_query)
	           		.bind(&res_format)
	           		.bind(0)
	           		.execute(&pool)
	           		.await
	           		.unwrap();
            });
        });

	let app = Router::new()
		.route("/api/health", get(health_check))
		.route("/api/*fn_name", post(server_fn_handler))
		.leptos_routes_with_handler(routes, get(leptos_routes_handler))
		.fallback(file_and_error_handler)
		.with_state(app_state)
		.layer(session_layer)
		.layer(trace_layer)
		.layer(middleware::from_fn(add_remote_addr_to_extensions))
		.layer(CompressionLayer::new())
		.layer(ServiceBuilder::new().layer(CatchPanicLayer::new()));
		
    let port_number = 9889_u16;
   	let addr = SocketAddr::from((
   	    Ipv4Addr::UNSPECIFIED,
   	    port_number,
   	));


	let cert_file = format!("{}/domain.cert.pem", ssl_path);
    let key_file = format!("{}/private.key.pem", ssl_path);
    let ssl_config = OpenSSLConfig::from_pem_file(cert_file, key_file).unwrap();

	// let config = axum_server::tls_rustls::RustlsConfig::from_pem_file(cert_file, key_file).await.unwrap();

	// let acceptor = state.axum_acceptor(state.default_rustls_config());
   	
	log::info!("Serving on https://{}", addr);
   	// axum_server::bind_rustls(addr, config)
   	axum_server::bind_openssl(addr, ssl_config)
   		// ::bind(addr)
   	
   		// .acceptor(acceptor)
	   	.serve(
	   		// app.into_make_service()
	   		app.into_make_service_with_connect_info::<SocketAddr>()
	  	)
	   	.await
	   	.unwrap();
}

pub async fn health_check() -> impl IntoResponse {
	StatusCode::OK
}
