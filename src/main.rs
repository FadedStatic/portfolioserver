use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_files as fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ErrRespStruct {
    success: bool,
    message: String,
}

async fn ping_handler() -> impl Responder {
    HttpResponse::Ok().body("pong!")
}

async fn error_handler() -> impl Responder {
    let error_response = ErrRespStruct {
        success: false,
        message: String::from("404"),
    };

    HttpResponse::NotFound()
        .content_type("application/json")
        .json(error_response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();

    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(move || {
        App::new()
        .route("/ping", web::get().to(ping_handler))
        .service(fs::Files::new("/", "./static")
            .index_file("index.html")
        )
        .default_service(web::route()
            .guard(actix_web::guard::Get())
            .to(error_handler)
        )
    })
    .bind_openssl("0.0.0.0:443", builder)?
    .run()
    .await
}