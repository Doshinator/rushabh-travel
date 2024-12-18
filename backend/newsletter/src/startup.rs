use actix_web::{get, web::Form, App, HttpResponse, HttpServer, Responder};
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

#[get("/user")]
pub async fn get_user(_user: Form<User>) -> impl Responder {
    HttpResponse::Ok()
        .body("user")
}

#[derive(Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
}

// The main reason for using methods instead of functions is for organization, 
// in addition to providing method syntax and not having to repeat the type of self in every method’s signature
impl User {
    // ithin an impl block, the type Self is an alias for the type that the impl block is for
    pub fn build_user() -> User {
        User {
            first_name: String::from("George"), 
            last_name: String::from("Armani")
        }
    }

    pub fn build_custom_user(first_name: String, last_name: String) -> User {
        User {
            first_name,
            last_name,
        }
    }
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
