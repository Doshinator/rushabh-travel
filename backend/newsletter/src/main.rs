// !src/main

// use std::{fs::File, io::ErrorKind};
use newsletter::startup::run;
use newsletter::errors::open_file;

#[tokio::main]
async fn main() -> std::io::Result<()>  {
    run().await
}
