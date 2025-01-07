#![allow(unused)]
pub fn vector_iterate() {
    let mut v: Vec<i32> = Vec::new();
    v.push(23);
    v.push(4);
    v.push(8);
    for (i, el) in v.iter().enumerate() {
        print!("{} ", v[i]);
    }
    println!()
}

pub fn access_el_vector_1() {
    let v = vec![1, 2, 3, 4, 5];
    let i = 2;
    println!("V[{}] = {}", i, &v[i]);
}

pub fn access_el_vector_2() {
    let mut a = vec![2, 4, 3, 0, -1];
    for i in &a {
        print!("{} ", i);
    }
    println!()
}