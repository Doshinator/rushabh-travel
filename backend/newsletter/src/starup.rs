//! src/startup.rs
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build() {
        run()
    }

}

pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/hello_world", web::get.to(hello_world))
        .bind("127.0.0.1:8000")?
        .run()
        .await
    })
} 
