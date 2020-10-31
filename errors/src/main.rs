use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::io::File;

// Example of propagating errors to the caller.
fn read_username_from_file() -> Result<String, io::error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn read_username_from_file2() -> Result<String, io::error> {
    let f = File::open("hello.txt")?; // ? operator automatically propagates
                                      // errors while returning the Ok(xyz) case
                                      // if no error.
    let s = String::new();
    f.read_to_string(&mut s)?; // ? operator automatically propagates errors
                               // while returning the Ok(xyz) case if no error.
    Ok(s)
}
fn read_username_from_file3() -> Result<String, io::error> {
    let s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?; // ? operator automatically
                                                      // propagates errors while
                                                      // returning the Ok(xyz)
                                                      // case if no error.
    Ok(s)
}
fn read_username_from_file4() -> Result<String, io::error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    // A OR B
    // let f = File::open("hello.txt"); // Returns Result<T, E>
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };
    // let f = File::open("hello.txt").unwrap() // Same function as above sample
    let f = File::open("hello.txt").expect("Failed to open hello.txt") // If err, panics with the provided message.

}
