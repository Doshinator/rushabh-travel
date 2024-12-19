use actix_web::{get, web::{Path, Query}, App, HttpResponse, HttpServer, Responder};
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


/**
 * 
 *  Path<>
 * 
 */
// if there a required field and was not provided, the request will fail with a 400 Bad Request or similar error.
// you NEED to have required field in the handler!!!
#[derive(Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
}

// need to have {first_name} and {last_name} as path REQUIRES you to have field in handler
#[get("/user/{first_name}/{last_name}")]
pub async fn get_user_path(user: Path<User>) -> impl Responder {
    let u = user.into_inner();
    HttpResponse::Ok()
        .body(format!("first_name={} & last_name={}", u.first_name, u.last_name))
}

/**
 * 
 *  Query<>
 *  You HAVE to have the members in the handler, else make them optional or default values
 * 
*/

// 1) With required member
#[derive(Deserialize)]
pub struct User1 {
    // you HAVE to call first_name & last_name in handler & in the http GET call.
    first_name: String,
    last_name: String,
}

#[get("/user1")]
pub async fn get_user1_query(user: Query<User1>) -> impl Responder {
    let u = user.into_inner();
    HttpResponse::Ok()
        .body(format!("User1 : first_name={} & last_name={}", u.first_name, u.last_name))
}


// 2) With Option
#[derive(Deserialize)]
pub struct User2 {
    first_name: String,
    // you DONT have to call first_name & last_name in handler & in the http GET call.
    last_name: Option<String>,
}
#[get("/user2")]
pub async fn get_user2_query(user: Query<User2>) -> impl Responder {
    let u = user.into_inner();
    
    HttpResponse::Ok()
        .body(format!("first_name={} & last_name={}", 
            u.first_name,
            u.last_name.unwrap_or_else(|| "Default b/c of no input".to_string())))
}

// 3) With Default
#[derive(Deserialize)]
pub struct User3 {
    first_name: String,
    // you DONT have to call first_name & last_name in handler & in the http GET call.
    #[serde(default = "get_default_last_name")]
    last_name: String,
}

pub fn get_default_last_name() -> String {
    "N/A".to_string()
}

#[get("/user3")]
pub async fn get_user3_query(user: Query<User3>) -> impl Responder {
    let u = user.into_inner();

    HttpResponse::Ok()
        .body(format!("first_name={} & last_name={}", u.first_name, u.last_name))
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
            .service(get_user1_query)
            .service(get_user2_query)
            .service(get_user3_query)
            // .service(get_user_query)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
