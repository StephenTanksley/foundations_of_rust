// EXAMPLE - using the panic! macro.

// fn main() {
//     panic!("crash and burn");
// }

// Stack Tracing - So if there are nested functions, it can be hard to tell
// which of those functions is actually being called and where. This is where
// the stack trace comes in. If we call RUST_BACKTRACE=1 cargo run, we can
// enable the backtrace which will show the chain of functions being called
// to show us the entire lineage of that function being invoked.
// fn main() {
//     a();
// }

// fn a() {
//     b();
// }

// fn b() {
//     c(21)
// }

// fn c(num: i32) {
//     if num == 22 {
//         panic!("Don't pass in 22!");
//     }
// }

// The Result enum
/*
    The Result enum has two options (just like the Option enum) - Ok, Err.
    Ok represents some generic success and stores a successful value,
    Err represents some generic error and stores an error value.

    This enum and its two variants are brought into scope by default since
    it is very common.
*/
// fn main() {
//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }
// }

// use std::fs::File;
// use std::io::ErrorKind;

// Attempt #1 - panic when you hit an error
// fn main() {
//     let f = File::open("hello.txt");

//     let f: File = match f {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {:?}", error),
//     };
// }

// Attempt #2 - match on the type of error you get to handle it more gracefully.
//  This version works, but nested match statements feel very messy and could become
//  hard to read and maintain.

// fn main() {
//     let f = File::open("hello.txt");

//     let f: File = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error)
//             }
//         },
//     };
// }

// fn main() {
//     let f: File = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }

// use std::fs::File;

// fn main() {
//     // These are equivalent processes, but this first one doesn't require the use of the match expression.
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");

//     // let f: File = match f {
//     //     Ok(file) => file,
//     //     Err(error) => panic!("Problem opening the file: {:?}", error),
//     // };
// }

// use std::fs::{self, File};
// use std::io;
// use std::io::Read;

// fn read_username_from_file() -> Result<String, std::io::Error> {
//     // // This does something similar to the .unwrap() or .expect() methods.
//     // // If it encounters an error, it doesn't panic. Instead, it will
//     // // end early and return the error.
//     // let mut f = File::open("hello.txt")?;

//     // // We use the comma to terminate match expressions
//     // // let mut f = match f {
//     // //     Ok(file) => file,
//     // //     Err(e) => return Err(e),
//     // // };

//     // let mut s: String = String::new();

//     // // Likewise can can simplify the match expression below...
//     // // match f.read_to_string(&mut s) {
//     // //     Ok(_) => Ok(s),
//     // //     Err(e) => Err(e),
//     // // }

//     // f.read_to_string(&mut s)?;
//     // Ok(s)

//     // We can simplify this further by chaining method calls.
//     let mut s: String = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)

//     // Can it get __even__ simpler? Yes, yes it can.
// }

// use std::error::Error;
// use std::fs::File;

// fn main() -> Result<(), Box<dyn Error>> {
//     let f: File = File::open("hello.txt")?;
//     Ok(())
// }

use std::net::IpAddr;
fn main() {
    // In the case of something where we know for a fact that it will succeed,
    // it's fine to unwrap. Otherwise if there's a possibility of it failing,
    // we'd want to handle the potential error case by using .expect()
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{}", home);
}
