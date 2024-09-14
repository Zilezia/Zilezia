use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web_lab::web::spa;
use mysql::*;
use mysql::prelude::*;

use common::config::{get_ip, get_mysql_url, get_port, get_table, load_env};
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

    let ip = get_ip();
    let port = get_port().parse::<u16>().unwrap();

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
    .bind((ip, port))?
    .run()
    .await
}
