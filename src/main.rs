use std::fs::File;
use std::fs::read_to_string;
// use std::io::ErrorKind;
use std::io;
use std::error::Error;
// use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
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
    /* let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|create_error| {
                panic!("Error creating file {:?}", create_error)
            })
        } else {
            panic!("Could not open the file: {:?}", error)
        }
    });
     */

    // let f = File::open("hello.txt").expect("Could not open the file hello.txt");

    // 5
    // match read_username_from_file() {
    //     Ok(username) => println!("the username is {}", username),
    //     Err(e) => panic!("Error getting username. {:?}", e)
    // }

    // 6 The next line will be blocked by the compiler. The ? is only valid in a function that
    // returns a Result or an Option.
    // We can change the function header of this main method to:
    // fn main() -> Result<(), Box<dyn Error>> {
    // So we can use the ? operator in main.
    let f = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    // 5.1 First version of this feature implementation
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };
    //
    // let mut s = String::new();
    //
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e)
    // }

    // 5.2 better implementation with the ? operator
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // 5.3 even shorter
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // 5.4 even more shorter
    read_to_string("hello.txt")
}
