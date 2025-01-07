#![allow(unused)]
pub fn create_empty_string() -> String {
    String::new()
}

pub fn string_slices() -> String {
    let s = "initial sliced string";
    s.to_string()
}


pub fn append_to_string() {
    let mut curr = String::from("foo");
    let appendage: &str = "bar";
    curr.push_str(appendage);
    curr.push('!');
    println!("appended string = {}", curr);
}

pub fn index_string() {
    let hello = String::from("Hello");
    for b in  hello.bytes() {
        print!("{} ", b);
    }
    println!();
}