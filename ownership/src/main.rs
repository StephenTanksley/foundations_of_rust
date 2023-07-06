// fn main() {
//     // let mut vec: Vec<i32> = vec![1, 2, 3];
//     let mut bits: i32 = 256;
//     println!("{:?}", bits);

//     let bits_plus_one = add_one(bits);
//     // I think this lower print statement should fail because
//     // I'm accessing something which should exist on the stack in F1.
//     // vec.push(4);
//     println!("{:?}", bits_plus_one);
// }

// fn add_one(x: i32) -> i32 {
//     println!("{}", x + 1);
//     x + 1
// }

// fn main() {
//     /*
//         For this example, we're creating a vector with
//         3 elements on the heap. We're then creating a
//         reference to the last element on the heap at
//         index 2.
//     */
//     let mut vec: Vec<i32> = vec![1, 2, 3];
//     let num: &i32 = &vec[2];

//     /*
//         In this next section, we push to that vector
//         to add a new element. This is the crux of it -
//         When we push to the vector, the current allocation
//         on the heap has no more space. In order to accommodate
//         the new element, the pointer to that current place in
//         the heap is dropped. New space is allocated on the heap
//         and the current vector plus the new item are copied into that
//         new location. This invalidates num - the pointer is no
//         longer valid.

//         Pointer safety: "data should never be aliased and mutated
//         at the same time."
//     */
//     // vec.push(4);
//     println!("The third element is {}", *num);
// }

/* I know that these items below won't work, don't @ me.
    v is invalidated because we push n to it. The vector
    runs out of space and needs to be copied to a new location
    so the pointer at v.remove is invalid.
*/
// fn give_and_take(v: &Vec<i32>, n: i32) {
//     v.push(n);
//     v.remove(index: 0);
// }

// fn main() {
//     let v = vec![1, 2, 3];
//     let n = &v[0];
//     give_and_take(&v, 4);
//     println!("{}", n);
// }

// fn main() {
//     let mut s1: String = String::from("hello");
//     change(&mut s1);
//     println!("{}", s1)
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

/*
    Rules of References
    -------------------
    1) At any given time, you can have either one mutable
    reference or any number of immutable references.

    2) References must always be valid. No dangling
    references are permitted.

*/

// fn main() {
//     let mut s: String = String::from("hello");

//     let r1: &String = &s;
//     let r2: &String = &s;

//     println!("{}, {}", r1, r2);
//     let r3: &mut String = &mut s;
//     println!("{}", r3);
// }

// SLICES
/*
    Slice syntax here is reminiscent of Python syntax but with
    what appears to be range operators instead of [:]  as you'd
    find in a Python slice. [0..5] and [..5] are equivalent.
    [6..11] and [6..] are equivalent...provided you want to have the
    slice capture the remainder of the string.

    Like in Python using the [:], [..] will slice and reference
    the whole string.
*/
// fn main() {
//     let s: String = String::from("hello world");
//     let s2: &str = "hello world";

//     let word: &str = first_word(s2);
//     println!("s: {}", first_word(&s));
//     println!("{}", word);
// }

// fn first_word(s: &str) -> &str {
//     let bytes: &[u8] = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b: (i32, char) = (25, 'a');
    let slice: &[i32] = &a[0..3];

    println!("{:?}", slice);
    println!("{:?}", b);
}