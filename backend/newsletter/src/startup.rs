use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tracing_actix_web::TracingLogger;
use uuid::Uuid;

#[get("/")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok()
        .body("Hello World.")
}

#[get("/name")]
pub async fn name() -> impl Responder {
    HttpResponse::Ok()
        .body("name")
}

#[get("/getid")]
pub async fn get_id() -> impl Responder {
    HttpResponse::Ok()
        .body(format!("Id={}", Uuid::new_v4().to_string()))
}



pub struct FirstName(String);

pub struct LastName(String);

// !src/startup.rs
pub async fn run() -> std::io::Result<()> {
    HttpServer::new( move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(hello_world)
            .service(name)
            .service(get_id)
            .service(get_user)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
