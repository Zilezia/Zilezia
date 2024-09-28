use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web_lab::web::spa;
use mysql::*;
use mysql::prelude::*;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use common::config::{load_env, get_mysql_url, get_table, get_ip};
use common::models::Activity;

// only "global" GET cuz i wanna edit the db directly using a bot
async fn get_activities() -> impl Responder {
    // loads of shit going on like that vvv
    let opts = match Opts::from_url(&get_mysql_url()) {
        Ok(opts) => opts,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database URL error: {}", e)),
    };
    let pool = match Pool::new(opts) {
        Ok(pool) => pool,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database pool error: {}", e)),
    };
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database connection error: {}", e)),
    };
    let query = format!(
        // all not work?
        "SELECT id, name, status, url, display 
        FROM {}", get_table());
    let result: Vec<Activity> = match conn.query_map(
        query,
        |(
            id,
            name,
            status,
            url,
            display
        )| Activity { 
            id,
            name,
            status,
            url,
            display 
        }
    ) {
        Ok(activs) => activs,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Query error: {}", e)),
    };

    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env();

    // let hip = get_ip();
    // let url = format!("{}:8080", hip);

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    acceptor.set_private_key_file("./back/private.key.pem", SslFiletype::PEM)?;
    acceptor.set_certificate_chain_file("./back/domain.cert.pem")?;

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/api", web::get().to(get_activities)) // this way may be bad, todo: swap to .service()
            .service(
                spa() // now runable from root, just ofc compile frontend
                .index_file("./front/dist/index.html") 
                .static_resources_mount("/")
                .static_resources_location("./front/dist")
                .finish()
            )
    })
    .bind_openssl("0.0.0.0:8080", acceptor)?
    .run()
    .await
}
