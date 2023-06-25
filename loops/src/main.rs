fn main() {
    // unbounded loop
    // loop {
    //     println!("again!");
    // }

    // Returning value from a loop. This changes a variable with a loop declaration in it and then returns that value.
    // let mut counter: i32 = 0;

    // let result: i32 = loop {
    //     counter += 1;
    //     println!("Current value is {counter}");
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // Labelled loops - such a cool idea
    // let mut count = 0;

    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");

    // let mut number: i32 = 3;

    // while number != 0 {
    //     println!("{number}");

    //     number -= 1;
    // }
    // println!("LIFTOFF!!!");

    // ACCESSING Array elements by iterator - error prone because we could have the wrong number of indexes.
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("The value is: {}", a[index]);

    //     index += 1;
    // }

    // FOR LOOP in Rust
    // let a = [10, 20, 30, 40, 50];
    // for element in a {
    //     println!("the value is: {element}");
    // }

    for number in (1..30).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}
