fn main() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let largest: i32 = get_largest(number_list);
    println!("The largest number is {}", largest);

    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    let largest: char = get_largest(char_list);
    println!("The largest number is {}", largest);
}

// Quick and easy function to find the largest integer from a vector of integers.
// fn get_largest(number_list: Vec<i32>) -> i32 {
//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

// So what if we wanted to get the largest char from a list of chars?
// First option would be literally to just duplicate the code, altering for chars.
// The second option is to refactor the code to use generics.
// The generic T is used after the function name. After T, we specify the traits
// we need. For example, not all types could be compared, so we can't just accept
// all types willy-nilly. By adding traits, we can allow parameters for this function:
// Any type which contain this combination of traits (PartialOrd and Copy) are permitted.
fn get_largest<T: PartialOrd + Copy>(char_list: Vec<T>) -> T {
    let mut largest = char_list[0];

    for character in char_list {
        if character > largest {
            largest = character;
        }
    }
    largest
}
