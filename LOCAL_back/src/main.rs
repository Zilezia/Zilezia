use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(Files::new("/", "../lain_wgl/").index_file("index.html"))
    })
    .bind(("192.168.1.250",8080))? // this laptop
    .run()
    .await
}
