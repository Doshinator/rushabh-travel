use std::{fs::File, io::ErrorKind};

// Error handling

// Result<T, E> 
// T: type of value returned if success
// E: type of error returned in case of failur

// to handle Result, use match.
// match experssion {
//     Ok(_) => _,
//     Err(e) => print!("{:?}", e),
// }

// Ok(inner_value) will return inner_value is success
// Err(e) will return e value if fail

// ----------- EXAMPLE -----------
// let file_handler = match File::open("hello.txt") {
//     Ok(file) => file,
//     Err(error) => match error.kind() {
//         ErrorKind::NotFound => match File::create("hello.txt") {
//             Ok(fc) => fc,
//             Err(e) => panic!("{e:?}"),
//         },
//         other_error => {
//             panic!("other type of error. {other_error:?}")
//         }
//     }
// };


pub fn open_file() {
    let file_handler = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create_new("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("{:?}", e),
            },
            other_error => {
                panic!("Unable to create file hello.txt. {:?}", other_error)
            }
        }
    };
}


// Alternative to using match with Result<T, E>
// ---- Closures ----

pub fn open_file_with_closure() {
    let file_handler = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create_new("hello.txt").unwrap_or_else(|e| {
                panic!("Problem creating the file: {:?}", e);
            })
        }
        else {
            panic!("Problem opening the file: {error:?}")
        }
    });
}



