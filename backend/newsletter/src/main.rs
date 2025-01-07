// !src/main

// use newsletter::startup::run;

// #[tokio::main]
// async fn main() -> std::io::Result<()>  {
//     run().await
// }

use newsletter::{hashmap, strings, vector};

fn main() {
    vector::vector_iterate();
    vector::access_el_vector_1();
    vector::access_el_vector_2();

   
    strings::append_to_string();
    strings::index_string();

    hashmap::two_sum();
}