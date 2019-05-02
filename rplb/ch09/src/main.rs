/*
 *  Error Handling
 *  Rust has two major categories of errors
 *  - recoverable: return a Result< T, E>
 *  - unrecoverable: panic! macro
 */

use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    /*
     *  unrecoverable errors with panic!
     */
    // by default, with panic! the program begins unwinding - rust walks back up
    // the stack and cleans up the data
    // can also abort and leave the cleaning up to the os
    //   this can be acheived through panic = 'abort'
    // panic!("crash and burn");
    // generate_panic();
    // kind of clunky code with so many match statements - Result have more
    // methods with it to clean things like this up
    let f1 = match File::open("hello1.txt") {
        Ok(file) => {
            println!("opened file successfully");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello1.txt") {
                Ok(fc) => {
                    println!("created file successfully");
                    fc
                }
                Err(e) => panic!("failed to create file: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    let f2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("failed to create file: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

    // .expect() lets us handle file open failure with our own error message
    let f3 = File::open("hello3.txt").expect("Failed to open file hello.txt");
}

// fn generate_panic() {
//     println!("wait for it...");
//     panic!("testing backtrace feature");
// }

fn read_username_from_file() -> Result<String, io::Error> {
    let f4 = File::open("hello4.txt");
    let mut f4 = match f4 {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f4.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// the ? operator can only be used within functions that return a Result<>
// this means it cannot be used within main, because main doesn't return a
// Result<>
fn read_file_better() -> Result<String, io::Error> {
    let mut f5 = File::open("hello5.txt")?;
    let mut s = String::new();
    f5.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file_even_better() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello6.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file_the_bestest() -> Result<String, io::Error> {
    fs::read_to_string("hello7.txt")
}
