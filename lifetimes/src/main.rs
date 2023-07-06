// LIFETIMES

/*
    Lifetimes refer to how long a variable lives within
    the scope of execution. In the following example, r
    is introduced (we'll refer to that lifetime as 'a) just
    below the main function. We then introduce a closure
    and create a new variable, x (referred to as 'b).

    'b only exists for the duration of its scope, so it
    dies out as we hit the closing curly bracket, taking the
    value assigned to r with it.

    When we attempt to access r again for the prinln! macro,
    we run into undefined behavior because we're trying to
    access something which has had its scope run out.
*/

// fn main() {
//     let r: &i32;

//     {
//         let x: i32 = 5;
//         r = &x;
//     }

//     println!("r: {}", r);
// }

/*
    In the case below, we do something similar, but we've set
    the code up without an interior closure. As a result, when
    r references the pointer to x, x remains valid as we get down
    to the println! macro. This program will compile.
*/
// fn main() {
//     let x: i32 = 5;

//     let r: &i32 = &x;

//     println!("r: {}", r);
// }

/*
    Let's take a look at the following example.
*/

/*
    Lifetime annotations -

    &i32       ----> a reference
    &'a i32    ----> a reference with an explicit lifetime
    &'a mut i32 ---> a mutable reference with an explicit lifetime
*/

// fn main() {
//     let string1: String = String::from("abcd");
//     let string2: String = String::from("xyz");

//     let result: &str = longest(string1.as_str(), string2.as_str());
//     println!("The longest string is {}", result);
// }

// /*
//     In the below example, what are we actually saying?
//     We're saying that there is a relationship between x, y, and 'a.
//     The lifetime of the returned reference will the same as the smallest
//     lifetime of the arguments.
// */
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// /*
//     For example, if we're specifying that there's a relationship between
//     the lifetimes of x, y and the returned reference, we're saying that
//     if x has the shortest lifetime, the returned reference's lifetime will
//     be equivalent to x, if y is the shortest then y.
// */
// VALID EXAMPLE BELOW
// fn main() {
//     let string1: String = String::from("abcd");

//     {
//         let string2: String = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         // valid code - this is valid because string2 exists within the same scope that
//         // it is called in within the println! macro.
//         println!("The longest string is {}", result);
//     }
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

//INVALID EXAMPLE BELOW

// fn main() {
//     let string1: String = String::from("abcd");
//     let result: &str;
//     {
//         let string2: String = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// The function below is going to break because returning a reference
// to data created within the function means that data will be
// destroyed at the end of the function's scope. We can only return
// a reference to data which was created outside the function's scope.

// To get around this, instead of returning a reference,
// you can just return the actual data by transferring ownership
// which can then be stored in a variable.
// INVALID EXAMPLE BELOW ---->
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     let result: String = String::from("really long string");
//     result.as_str()
// }

// VALID EXAMPLE BELOW ---->
// fn longest<'a>(x: &'a str, y: &str) -> String {
//     let result: String = String::from("really long string");
//     result
// }

/*
    In the below example, we define a struct ImportantExcerpt<'a>
    which features a part, defined as a borrowed string slice. In the
    main function, first_sentence is sliced from novel and then used
    to construct the variable i: ImportantExcerpt. By setting up the lifetime
    used for i, we explicitly are telling the borrow checker to look at the
    lifetime used for first_sentence. If first_sentence were to go out of scope,
    i would be invalid because we're tying it to the shortest lifetime of the
    passed arguments (in this case first_sentence).
*/
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// fn main() {
//     let novel: String = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence: &str = novel.split(".").next().expect("Could not find...");
//     let i: ImportantExcerpt = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// LIFETIME ELISION RULES

/*
    1. Each parameter that is a reference gets its own lifetime parameter.
    2. If there is exactly one input lifetime parameter, that lifetime is
       assigned to all output lifetime parameters;
    3. If there are multiple input lifetime parameters, but one of them is
       &self or &mut self, the lifetime of self is assigned to all output
       lifetime parameters.

    In the below example, the compiler is able to deduce the lifetime of s
    because there is only one input provided. Technically we could omit
    passing an explicit direction for lifetimes and the compiler could
    provide that on its own.

    The third rule refers more to methods as shown in the next set of examples...
*/
// fn main() {}

// fn first_word<'a>(s: &'a str) -> &'a str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// fn main() {
//     let novel: String = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence: &str = novel.split('.').next().expect("Could not find...");
//     let i: ImportantExcerpt = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// STATIC LIFETIME

/*
    Static lifetimes can have a duration as long as the program.
    All string literals have a static lifetime.
*/
// fn main() {
//     let s: &'static str = "I have a static lifetime.";
// }

// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {}
