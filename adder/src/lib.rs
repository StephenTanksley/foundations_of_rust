// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// pub struct Guess {
//     value: i32,
// }

// pub fn prints_and_returns_10(a: i32) -> i32 {
//     println!("I got the value {}", a);
//     10
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!(
//                 "Guess value must be greater than or equal to 1, got {}.",
//                 value
//             );
//         } else if value > 100 {
//             panic!(
//                 "Guess value must be less than or equal to 100, got {}.",
//                 value
//             );
//         }

//         Guess { value }
//     }
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// pub fn greeting(name: &str) -> String {
//     format!("Hello, {}!", name)
// }

// // Because this is technically a module, things outside the module itself
// // are considered to be part of the parent module, namely the Rectangle struct
// // and impl blocks above. To bring them into scope, we utilize the "use super::*;" line below.
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger: Rectangle = Rectangle {
//             width: 8,
//             height: 7,
//         };

//         let smaller: Rectangle = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger: Rectangle = Rectangle {
//             width: 8,
//             height: 7,
//         };

//         let smaller: Rectangle = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(!smaller.can_hold(&larger));
//     }

//     // In Rust, there is no specific convention about left/right sides of testing assertions
//     // having semantic meaning. As long as the tests pass, Rust doesn't care.
//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(2));
//         assert_ne!(5, add_two(2));
//     }

//     #[test]
//     fn greeting_contains_name() {
//         let result: String = greeting("Carol");
//         assert!(
//             result.contains("Carol"),
//             "Greeting did not contain name, value was '{}'",
//             result
//         );
//     }

//     #[test]
//     #[should_panic(expected = "Guess value must be less than or equal to 100, got 200")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }

//     #[test]
//     #[should_panic(expected = "Guess value must be greater than or equal to 1, got 0")]
//     fn less_than_1() {
//         Guess::new(0);
//     }

//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }

//     #[test]
//     fn this_test_will_pass() {
//         let value: i32 = prints_and_returns_10(4);
//         assert_eq!(10, value);
//     }

//     // To
//     #[test]
//     #[ignore]
//     fn this_test_will_fail() {
//         let value: i32 = prints_and_returns_10(8);
//         assert_eq!(5, value);
//     }
// }

// // The Rust community's convention for testing holds that tests are held in the same file
// // as your product code.

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
