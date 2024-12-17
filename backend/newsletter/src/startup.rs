use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tracing_actix_web::TracingLogger;

#[get("/")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok()
        .body("Hello World.")
}

// !src/startup.rs
pub async fn run() -> std::io::Result<()> {
    HttpServer::new( move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(hello_world)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}