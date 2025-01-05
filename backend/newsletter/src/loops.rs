pub fn condition() {
    let condition = true;
    let number = if condition { 
        5 
    } else {
        6
    };
    println!("The value of number is: {number}");
}

pub fn loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

pub fn while_loops() {
    let mut counter = 1;
    while counter < 32 {
        counter += 1;
    };
    println!("Final counter {}", counter);
}

pub fn fizz_buzz(counter: i32) {
    if counter < 1 {
        return;
    };

    let mut i = 1;
    while i <= counter {
        if i % 15 == 0 {
            println!("FizBuzz");
        }
        else if i % 3 == 0 {
            println!("Fizz");
        } 
        else if i % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{i}");
        }
        i += 1;
    };
}

pub fn fizz_buss_with_for_loop(counter: i32) {
    for i in 1..=counter {
        if i % 15 == 0 {
            println!("FizzBuzz");
        }
        else if i % 3 == 0 {
            println!("Fizz");
        }
        else if i % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", i);
        }
    }
}

pub fn loop_elements() {
    let a = vec![2, 1, 5, 7, 3, 9];
    for (i, _el) in a.iter().enumerate() {
        print!("{} ", &a[i]);
    }
}