use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // 1
    // Simply crash.
    // panic!("crash and burn");

    // 2
    // Crash when accessing index that is not defined ( index out of bounds).
    // This crash points to a file we didn't write because the panicking is happening there.
    // We have to set the environment variable RUST_BACKTRACE to 1
    // Use this command to run the programm: RUST_BACKTRACE=1 cargo run
    // let v = vec![1, 2, 3];
    // v[99];

    // 3
    // Crash when the file does not exists.
    /*
    let f = File::open("hello.txt");

    // Catching the error result by creating the file when it does not exists.
    // Panics when the error is another one.
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(createdFile) => createdFile,
                Err(err) => panic!("Error creating file {:?}", err)
            },
            other_error => panic!("Could not open file: {:?}", other_error)
        }
    };*/

    // 4
    // The same as 3, but more rustacean way of doing things (with closures).
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|create_error| {
                panic!("Error creating file {:?}", create_error)
            })
        } else {
            panic!("Could not open file: {:?}", error)
        }
    });

    // Continue here: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#shortcuts-for-panic-on-error-unwrap-and-expect
}
