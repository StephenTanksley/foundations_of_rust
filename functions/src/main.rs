fn main() {
    // another_function(5);
    // print_labeled_measurement(5, 'h');
    fn add_nums(num_one: i32, num_two: i32) -> i32 {
        num_one + num_two
    }
    let y = {
        let x: i32 = 4;
        // We DON'T add a semicolon at the end of a line if we want to include that value as a return.
        x + add_nums(2, 3)
    };

    println!("The value of y is: {y}")

}

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }