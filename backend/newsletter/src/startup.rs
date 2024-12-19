use actix_web::{get, web::Path, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
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

#[derive(Deserialize)]
pub struct User {
    first_name: String,
}

#[get("/user/{first_name}")] // {has_to_match_member_var_in_struct}
pub async fn get_user(user: Path<User>) -> impl Responder {
    let u = user.into_inner();
    HttpResponse::Ok()
        .body(format!("first_name={}", u.first_name))
}





// replace this later inside the User struct
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
