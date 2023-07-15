// GENERICS

// fn main() {
//     let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
//     let largest: i32 = get_largest(number_list);
//     println!("The largest number is {}", largest);

//     let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
//     let largest: char = get_largest(char_list);
//     println!("The largest number is {}", largest);
// }

// // Quick and easy function to find the largest integer from a vector of integers.
// // fn get_largest(number_list: Vec<i32>) -> i32 {
// //     let mut largest = number_list[0];

// //     for number in number_list {
// //         if number > largest {
// //             largest = number;
// //         }
// //     }
// //     largest
// // }

// // So what if we wanted to get the largest char from a list of chars?
// // First option would be literally to just duplicate the code, altering for chars.
// // The second option is to refactor the code to use generics.
// // The generic T is used after the function name. After T, we specify the traits
// // we need. For example, not all types could be compared, so we can't just accept
// // all types willy-nilly. By adding traits, we can allow parameters for this function:
// // Any type which contain this combination of traits (PartialOrd and Copy) are permitted.
// fn get_largest<T: PartialOrd + Copy>(char_list: Vec<T>) -> T {
//     let mut largest = char_list[0];

//     for character in char_list {
//         if character > largest {
//             largest = character;
//         }
//     }
//     largest
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: 5.0, y: 10.0 };
//     let p3 = Point { x: 5, y: 10.0 };
// }

// fn main() {
//     enum Option<T> {
//         Some(T),
//         None,
//     }

//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// // Implementation -
// // This is something that allows us to be more detailed in how our generics are implemented.
// // In the case of mixup, we're doing something here which blends the types of two points.
// // Our first point uses generic types T, U. Our second point uses generic types V, W.
// // By passing in self, we reference our Point<T, U>. We then call mixup which will return
// // a new Point<T, W> which uses the type provided for the second Point<V, W> to finally
// // create the final Point<T, W>.
// impl<T, U> Point<T, U> {
//     // fn x(&self) -> &T {
//     //     &self.x
//     // }

//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// // impl Point<f64> {
// //     fn y(&self) -> f64 {
// //         self.y
// //     }
// // }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };
//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// We don't actually have to specify different option enums
// since at compile time Rust will do this for us.

// What you see --->
// enum Option<T> {
//     Some(T),
//     None,
// }

// fn main() {
//     let integer = Option::Some(5);
//     let float = Option::Some(5.0);
// }

// What's actually happening under the hood --->
// enum Option_I32<i32> {
//     Some(i32),
//     None,
// }

// enum Option_F64<f64> {
//     Some(f64),
//     None,
// }

// fn main() {
//     let integer = Option_I32::Some(5);
//     let float = Option_F64::Some(5.0);
// }
