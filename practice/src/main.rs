// fn main() {
//     let mut vector: Vec<u32> = Vec::new();
//     let mut sum: u64 = 0;
//     for i in 1..1_000_000 {
//         if is_prime(i) {
//             sum += i as u64;
//             vector.push(i);
//         };
//     }
//     println!("Sum: {}", sum);
//     println!("Primes: {:?}", vector);
// }

// fn is_prime(n: u32) -> bool {
//     if n <= 1 {
//         return false;
//     }
//     for i in 2..(n as f64).sqrt() as u32 + 1 {
//         if n % i == 0 {
//             return false;
//         }
//     }
//     true
// }

// Fizz buzz implementation - very explicit, could be made more generic.

// fn main() {
//     for n in 1..=100 {
//         if n % 15 == 0 {
//             println!("fizz buzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n)
//         }
//     }
// }

// fn main() {
//     let some_value: Option<i32> = Some(3);

//     // Match expression - exhaustive, requires
//     // you to list out every possible variant.
//     match some_value {
//         Some(3) => println!("three"),
//         _ => (),
//     }

//     // if let syntax - functionally the same as
//     // match expression, but only for the case
//     // you actually care about.
//     if let Some(3) = some_value {
//         println!("three");
//     }
// }

// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     Arizona,
//     Arkansas,
//     California,
//     //...
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// fn main() {
//     let five: Option<i32> = Some(5);
//     let six: Option<i32> = plus_one(five);
//     let none: Option<i32> = plus_one(None);
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//         _ => None,
//     }
// }

// fn main() {
//     let some_value: Option<i32> = Some(3);
//     match some_value {
//         Some(3) => println!("three"),
//         _ => (),
//     }

//     if let Some(3) = some_value {
//         println!("three");
//     }
// }

// fn main() {
//     let some_value: Option<i32> = Some(3);
//     let _some_bool: Option<bool> = None;
//     let some_other_bool: Option<bool> = Some(true);

//     // if-let syntax - very handy for specifying something discrete;
//     if let Some(3) = some_value {
//         println!("three");
//     }

//     if let Some(true) = some_other_bool {
//         println!("true");
//     } else {
//         println!("false");
//     }
// }

// Additional learning material from Let's Get Rusty
// https://www.youtube.com/watch?v=DSZqIJhkNCM&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=6

// fn main() {
//     let mut x: Box<i32> = Box::new(1);
//     // a is dereferenced - that is to say that it is accessing what's stored on the heap
//     // by the variable x (1)
//     let a: i32 = *x;
//     // This actually modifies the heap value
//     *x += 1;
//     // println!("{}", x);

//     // We define a variable r1 which is a borrowed Box that contains a signed 32-bit integer.
//     // We then assign that reference of x which exists on the stack...which itself references a value on the heap.
//     let r1: &Box<i32> = &x;
//     let b: i32 = **r1; // to access the value at x from r1, we have to dereference twice.

//     let r2: &i32 = &*x;
//     let c: i32 = *r2;
// }

// A site/blog here for digging into the usage of Rust for data engineering.
// https://datawithrust.com/

// A book full of techniques for profiling/optimizing Rust code.
// https://nnethercote.github.io/perf-book/general-tips.html

// fn main() {
//     let mut console_high_score: i32 = 8999;
//     {
//         let y = &mut console_high_score;
//         *y += 1;
//     }

//     println!("console_high_score is now {}", console_high_score);
// }

// use std::io;

// fn main() {
//     println!("What is your name?");

//     let mut name = String::new();
//     io::stdin()
//         .read_line(&mut name)
//         .expect("Failed to read input");

//     println!("Hello, {}, welcome to the world of Rust!", name.trim());
// }

fn main() {
    let mut count: i32 = 0;
    // loop {
    //     count += 1;
    //     if count == 10 {
    //         println!("We've arrived at the station and reached the end. Goodbye!");
    //         break;
    //     } else {
    //         let remaining_stations = 10 - &count;
    //         println!("There are {} stations remaining on your trip.", remaining_stations);
    //         continue;
    //     }
    // }

    while count < 10 {
        count += 1;

        if count == 10 {
            println!("We've arrived at the station!");
            break;
        }
        let remaining: i32 = 10 - &count;
        println!("There are {} stations remaining on your trip.", remaining);
    }
}
