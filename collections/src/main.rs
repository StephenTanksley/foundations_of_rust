// use unicode_segmentation::UnicodeSegmentation;

// fn main() {
//     let a: [i32; 3] = [1, 2, 3];
//     let mut v: Vec<i32> = Vec::new();

//     v.push(1);
//     v.push(2);
//     v.push(3);
//     {
//         let mut v2: Vec<i32> = vec![1, 2, 3];
//     }

//     println!("{:?}", v)
// }

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
//     let third: &i32 = &v[2];
//     v.push(6);
//     println!("The third element is {}", third);

//     match v.get(20) {
//         Some(&i32) => println!("The third element is {}", third),
//         None => println!("There is no third element"),
//     }
// }

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

//     for i in &mut v {
//         *i += 50;
//     }

//     for i in &v {
//         println!("{}", i);
//     }
// }

// Fun With Enums
/*
    Vectors can only store a single type of data within them.
    Unlike tuples, you can't mix and match i32, f64, char, String, etc.
    However you /can/ create an enum which has mixed types for fields and then
    store instances of the enum because they'll technically all be of the same type.
*/

// fn main() {

// // Say we want to represent a single cell in a spreadsheet.
// // We can represent a cell by creating an enum which lists
// // all the potential variations for this custom type.
// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// let row: Vec<SpreadsheetCell> = vec![
//     SpreadsheetCell::Int(3),
//     SpreadsheetCell::Text(String::from("blue")),
//     SpreadsheetCell::Float(10.12),
// ];

// // There's a catch for us doing it like this. You have to use a match expression
// // to check the type of each cell. For example, if we wanted to check a value within...

// match &row[1] {
//     SpreadsheetCell::Int(i: &i32) => println!("{}", i),
//     _ => println!("Not an integer!");
// };

// // We could then pair this with a struct to create a SpreadsheetRow composed of cells.
// }

// // Strings
// fn main() {
//     // let s1: String = String::new();
//     // // this is a string slice, not the same thing as s1
//     // let s2: &str = "initial contents";
//     // let s3: String = s2.to_string();
//     // let s4: String = String::from("initial contents");

//     // let mut s: String = String::from("foo");
//     // s.push_str(string: "bar");
//     // s.push(ch: '!');

//     // let s5: String = String::from("Hello, ");
//     // let s6: String = String::from("world!");

//     // // This last string takes ownership of s5 and then takes a
//     // // reference to the characters in s6 rather than owning it.
//     // // This saves a bit of memory compared to copying both strings.
//     // let s7: String = s5 + &s6;

//     // // This one's a bit different. Using the format macro actually
//     // // doesn't take ownership of s5 and s6
//     // let s8: String = format!("{}{}", s5, s6);

//     // Accessing items within strings
//     // Rust uses UTF-8 for character encoding. Very popular and universal.
//     // https://en.wikipedia.org/wiki/UTF-8

//     // Note that by using &str during creation, we're creating a slice.
//     let youre_welcome: &str = "Пожалуйста";
//     /*  For chars which exist in other languages than English, we can't assume
//         that indexes are going to work quite the same way. It's a better idea to
//         iterate over the actual bytes when the language is something other than
//         English (the above example is Russian).
//     */
//     for b in youre_welcome.bytes() {
//         println!("{}", b);
//     }

//     /*
//         Likewise we can also access the scalar values by using the .chars() method.
//     */
//     for c in youre_welcome.chars() {
//         println!("{}", c)
//     }

//     /*
//         Grapheme clusters - need to bring in a crate for this: unicode-segmentation. Scroll
//         to the top of the file to see it brought into scope via "use" keyword.
//     */
//     for g in youre_welcome.graphemes(true) {
//         println!("{}", g);
//     }

// }

// use std::collections::HashMap;

// //// Hashmaps
// fn main() {
//     let blue: String = String::from("Blue");
//     let yellow: String = String::from("Yellow");

//     let mut scores: HashMap<String, i32> = HashMap::new();
//     // We aren't passing by reference here - we're actually transferring
//     // ownership of blue and yellow to the hashmap.
//     // scores.insert(blue, 10);
//     // scores.insert(yellow, 50);

//     // // println!("{}", blue);

//     // let team_name: String = String::from("Blue");
//     // let score: Option<&i32> = scores.get(&team_name);

//     // for (key, value) in &scores {
//     //     println!("{}: {}", key, value);
//     // }

//     // There's an issue with the code below. Instead of appending,
//     // we're actually going to be overwriting the value at this key.
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Blue"), 20);

//     // The .or_insert() method gets around some of this. If a key
//     // doesn't exist, it will insert the key with the provided value.
//     // If it does exist, though, it won't do anything.
//     scores.entry(String::from("Yellow")).or_insert(30);
//     scores.entry(String::from("Yellow")).or_insert(40);
// }

use std::collections::HashMap;

fn main() {
    let text: &str = "hello world wonderful world";

    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
