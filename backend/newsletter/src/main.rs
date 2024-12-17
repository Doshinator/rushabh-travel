// !src/main

use newsletter::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()>  {
    run().await
}
