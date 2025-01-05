use std::{fs::File, io::ErrorKind};
use std::io::{self, Read};

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

// unwrap_or_else
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

// unwrap and expect
// If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
// If the Result is the Err variant, unwrap will call the panic! macro for us
pub fn open_file_unwrap() {
    let file_handler = File::open("hello.txt").unwrap();
}

// Using expect instead of unwrap and providing good error messages can 
// convey your intent and make tracking down the source of a panic easier
pub fn open_file_expect() {
    let file_handler = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}


// ---- Propogating Errors ---- 
pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file_handler = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match username_file_handler.read_to_string(&mut username) {
        Ok(_) => username,
        Err(e) => Err(e),
    }
}

// read_username_from_file can fail at many spots, so we propogate errors up using '?'
// same fn as above but with ?
pub fn read_username_from_file_propogate_error() -> Result<String, io::Error> {
    // error values that have the ? operator called on them go through the from function, 
    // defined in the From trait in the standard library, which is used to convert values from one type into another
    
    // When the ? operator calls the from function, the error type received is converted into the error type defined 
    // in the return type of the current function.
    // ie. ? goes through from function, which converts into io::Error type for this function
    let mut username_file_handler = File::open("hello.txt")?;
    let mut username = String::new();
    username_file_handler.read_to_string(&mut username)?;
    Ok(username)
}

pub fn shorter_read_username_from_file_propogate_error() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
// ? can only be used in funcitons that return Result<T, E>
