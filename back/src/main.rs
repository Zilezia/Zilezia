use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web_lab::web::spa;
use mysql::*;
use mysql::prelude::*;

use common::models::User;
use common::config::{get_api_route, get_ip, get_mysql_password, get_port, load_env};

async fn get_users() -> impl Responder {
    // let mysql_psw = env::var("PASSWORD").unwrap();
    let mysql_password = get_mysql_password();
    let url = format!("mysql://root:{}@localhost:3306/rust_db", mysql_password);
    let opts = match Opts::from_url(&url) {
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

    let result: Vec<User> = match conn.query_map(
        "SELECT id, username FROM users",
        |(id, username)| User { id, username }
    ) {
        Ok(users) => users,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Query error: {}", e)),
    };

    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env();

    let ip = get_ip();
    let port = get_port();
    
    HttpServer::new(|| {
        let api_route = get_api_route();
        App::new()
            .wrap(Cors::permissive())
            .route(api_route, web::get().to(get_users))
            .service(
                spa()
                .index_file("./dist/index.html")
                .static_resources_mount("/")
                .static_resources_location("./dist")
                .finish()
            )
    })
    .bind((ip,port))?
    .run()
    .await
}
