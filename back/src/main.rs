// learned a bit more how to use actix


use actix_cors::Cors;
use actix_files::{
    Files,
    NamedFile
};
use actix_web::{
    web::{self, Data},
    App,
    HttpServer,
    HttpResponse,
    Responder,
    middleware
};
use actix_web_lab::extract::Path;

use openssl::ssl::{
    SslAcceptor, 
    SslFiletype,
    SslMethod
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    acceptor.set_private_key_file("./back/ssl/private.key.pem", SslFiletype::PEM)?;
    acceptor.set_certificate_chain_file("./back/ssl/domain.cert.pem")?;

    log::info!("R");

    HttpServer::new(|| {
        let cors = Cors::permissive();
        /* also for later
            .allowed_methods(vec!["GET","POST"])
        */
        App::new()
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(Files::new("/", "./back/dist").index_file("index.html"))
            .default_service(web::route().to(|| async {
                NamedFile::open("./back/dist/index.html").unwrap()
            }))
    })
    .bind_openssl("0.0.0.0:8443", acceptor)?
    .workers(2)
    .run()
    .await
}
