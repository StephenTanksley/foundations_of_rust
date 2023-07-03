// EXAMPLE
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn server_order() {}

//         fn take_payment() {}
//     }
// }

/*
    PUB keyword - To hide implementation details, parent modules are
    unable to see into child modules, but child modules can view
    items from parents. In this case without the pub keyword before hosting,
    the eat_at_restaurant function is unable to view hosting, so it doesn't
    know where to go to look for the appropriate method. We have to deliberately
    expose both hosting and add_to_waitlist.
*/
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// super - we use super to access functions from the parent module.
// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal: crate::back_of_house::Breakfast = back_of_house::Breakfast::summer("Rye");

//     meal.toast = String::from("Wheat");
// }

// use crate::back_of_house::Appetizer;

// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let order1: Appetizer = back_of_house::Appetizer::Soup;
//     let order2: Appetizer = back_of_house::Appetizer::Salad;
// }

// // We utilize the "use" keyword to bring paths into scope.
pub use crate::front_of_house::hosting;
use rand::{CryptoRng, ErrorKind::Transient, Rng};

use std::io::{self, Write};
/*
    This is functionally identical to:
    use std::io;
    use std::io::Write;
*/

// To bring in all public items from a module into scope:
use std::io::*;

mod front_of_house;

/*
    Instead of having to specify "front_of_house::hosting::add_to_waitlist()",
    we can bring front_of_house::hosting into scope and remove front_of_house.

    We technically could drill down all the way to just add_to_waitlist, but
    it is considered good practice to bring the parent of a function into function
    usage declarations because we want to show that it comes from an external crate.

    If you're bringing enums or structs into scope, though, it is considered idiomatic
    to specify the full path, i.e. crate::back_of_house::Appetizer or crate::back_of_house::Breakfast;
*/
pub fn eat_at_restaurant() {
    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);
    hosting::add_to_waistlist();
    hosting::add_to_waistlist();
    hosting::add_to_waistlist();
}

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     Ok(())
// }

// fn function2() -> IoResult<()> {
//     Ok(())
// }
