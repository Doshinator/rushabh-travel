use actix_web::{get, post, web::{Json, Path, Query}, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use tracing_actix_web::TracingLogger;
use unicode_segmentation::UnicodeSegmentation;
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

// http://localhost:8000/user1?first_name=John&last_name=Doe
// need both in this case, otherwise it will 400
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

// http://localhost:8000/user2?first_name=John
// last_name is Optional 
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

// http://localhost:8000/user3?first_name=John
// last_name is Optional 
#[get("/user3")]
pub async fn get_user3_query(user: Query<User3>) -> impl Responder {
    let u = user.into_inner();

    HttpResponse::Ok()
        .body(format!("first_name={} & last_name={}", u.first_name, u.last_name))
}


// replace this later inside the User struct
#[derive(Debug)]
#[derive(Deserialize)]
pub struct FirstName(String);

#[derive(Debug)]
#[derive(Deserialize)]
pub struct LastName(String);

// 4) With Tuple struct
#[derive(Deserialize)]
pub struct User4 {
    first_name: FirstName,
    last_name: LastName,
}

#[get("/user4")]
pub async fn get_user4_query(user: Query<User4>) -> impl Responder {
    let u = user.into_inner();
    let first_name = FirstName(String::from(u.get_first_name()));
    let last_name = LastName(String::from(u.get_last_name()));

    HttpResponse::Ok()
        .body(format!("first_name={} & last_name={}", first_name.0, last_name.0))
}

impl User4 {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self {
            first_name: FirstName(first_name),
            last_name: LastName(last_name),
        }
    }

    pub fn get_first_name(&self) -> &str {
        &self.first_name.0
    }

    pub fn get_last_name(&self) -> &str {
        &self.last_name.0
    }
}

impl FirstName {
    pub fn parse(first_name: String) -> Result<FirstName, String> {
        let is_empty_or_white_space = first_name.trim().is_empty();
        let is_too_long = first_name.graphemes(true).count() > 15;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_character = first_name.chars().any(|g| forbidden_characters.contains(&g));
        
        if is_empty_or_white_space || is_too_long || contains_forbidden_character {
            Err(format!("{} is not a valid first_name", first_name))
        } else {
            Ok(Self(first_name))
        }
    }
}

/**
 * 
 *  POST
 * 
 */

#[derive(Deserialize)]
pub struct User5 {
    first_name: String,
    last_name: String,
}

// curl -X POST http://localhost:8000/users -H "Content-Type: application/json" -d '{"first_name": "Alice", "last_name": "Smith"}'
#[post("/users")]
pub async fn create_user(user: Json<User5>) -> impl Responder {
    let first_name = FirstName(user.first_name.clone());
    let last_name = LastName(user.last_name.clone());
    HttpResponse::Ok()
        .body(format!("User create: {}, {}", first_name.0, last_name.0))
}


#[derive(Deserialize)]
pub struct User6 {
    first_name: FirstName,
    last_name: LastName,
}

// curl -X POST http://localhost:8000/user6 -H "Content-Type: application/json" -d '{"first_name": "Alice", "last_name": "Smith"}'
#[post("/user6")]
pub async fn add_user(user: Json<User6>) -> impl Responder {
    let user6 = user.into_inner(); // user.into_inner() == user.0
    let first = user6.first_name;
    let last = user6.last_name;

    HttpResponse::Ok()
        .body(format!("User added: {}, {}", first.0, last.0))
}


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
            .service(get_user4_query)
            .service(create_user)
            .service(add_user)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
