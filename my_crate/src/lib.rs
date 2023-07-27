//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain calculations more convenient.
// ---> The above comments are a higher level summary of the crate itself.

// ---> The below comments are on the content of the crate.
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(answer, 6);
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 2
}
