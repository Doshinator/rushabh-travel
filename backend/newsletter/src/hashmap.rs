use std::collections::HashMap;

pub fn two_sum() {
    print!("Two Sum: ");
    let v = vec![2, 5, 3, 1, 7, 11];
    let target = 9;
    let mut m: HashMap<&i32, usize> = HashMap::new();

    for (i, num) in v.iter().enumerate() {
        if !m.contains_key(&(target - num)) {
            m.insert(num, i);
        }
        else {
            let val = m.get(&(target - num));
            match val {
                Some(n) => println!("index:{}, val:{} ", i, n),
                None => todo!(),
            };
        }
    }

    println!();
    for (val, i) in m.iter() {
        print!("key:{}, val:{} ", val, i);
    }
}