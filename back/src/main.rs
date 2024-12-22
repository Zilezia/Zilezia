// learned a bit more how to use actix
use actix_cors::Cors;
use actix_files::{ Files, NamedFile };
use actix_web::{ web, App, HttpServer };
use actix_web::middleware::{ Compress, Logger };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    acceptor.set_private_key_file("./back/ssl/private.key.pem", SslFiletype::PEM)?;
    acceptor.set_certificate_chain_file("./back/ssl/domain.cert.pem")?;

    let host = "0.0.0.0";
    let port = "8443";
    let addr = format!("{}:{}", host, port);

    log::info!("Server running on {}", addr);
    HttpServer::new(|| {
        let cors = Cors::permissive();
        /* for later maybe
            .allowed_methods(vec!["GET","POST"])
        */
        App::new()
            .wrap(cors)
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(Files::new("/", "./front/dist").index_file("index.html"))
            // just in case
            .default_service(web::route().to(|| async {
                NamedFile::open("./front/dist/index.html").unwrap()
            }))
    })
    .bind_openssl(addr, acceptor)?
    .workers(2)
    .run()
    .await
}
