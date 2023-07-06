// // Traits

// pub struct NewsArticle {
//     pub author: String,
//     pub headline: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     // fn summarize(&self) -> String {
//     //     format!("{}, by {}", self.headline, self.author)
//     // }
//     fn summarize_author(&self) -> String {
//         format!("{}", self.author)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }

//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// pub trait Summary {
//     // This first version is a bare-bones example.
//     // fn summarize(&self) -> String;

//     // This second version includes a default implementation.
//     // fn summarize(&self) -> String {
//     //     String::from("(Read more...)")
//     // }

//     fn summarize_author(&self) -> String;

//     fn summarize(&self) -> String {
//         format!("(Read more from {}...)", self.summarize_author())
//     }
// }

// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     }
// }

// // This is a super neat use of Traits. By using &impl, you're specifying
// // that you'll accept any type which implements the trait Summary.
// // Realistically we could call this for both Tweet and NewsArticle.
// // This is the impl syntax.

// // simple syntax --->
// // pub fn notify(item: &impl Summary) {

// // // more complex syntax --->
// // pub fn notify(item: &(impl Summary + Display), item2: &impl Summary) {
// //     println!("Breaking news! {}", item.summarize());
// // }

// // // There's another way of doing this -
// // // impl is syntactic sugar for something called trait-bound syntax.
// // // When we do this, we're specifying that notify's arguments MUST be
// // // type T which implements Summary. We could also add more traits if
// // // we need a more complex type. The signature would look like this:
// // // pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {};

// // // pub fn notify<T: Summary>(item1: &T, item2: &T) {
// // //     // Something here.
// // // }

// // // A note on readability -
// // // Trait definitions can get kinda unwieldy and non-readable if
// // // each trait needs to be very specific. For example... --->

// // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// //     // ...
// // }

// // // To address this, there exists the where clause. In this clause,
// // // we just bring down the specific type definitions to a lower line
// // // so the signature itself remains readable.

// // fn some_functions<T, U>(t: &T, u: &U) -> i32
// // where
// //     T: Display + Clone,
// //     U: Clone + Debug,
// // {
// //     // ...
// // }

// fn main() {
//     let tweet = Tweet {
//         username: String::from("@johndoe"),
//         content: String::from("hello world"),
//         reply: false,
//         retweet: false,
//     };

//     let news_article = NewsArticle {
//         author: String::from("John Doe"),
//         headline: String::from("The Sky is Falling!"),
//         content: String::from("...The sky is not actually falling."),
//     };

//     notify(&news_article);
//     // println!("Tweet summary: {}", tweet.summarize());
//     // println!("Article summary: {}", news_article.summarize());
// }

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
