fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");

    // x = 6;
    // println!("The value of x is: {x}");
    // Shadowing - re-binding a variable to incorporate a former variable's value. 
    let x = 5;
    
    // Shadowed to now have x be the result of x(5) + 1 => 6;
    let x = x + 1;

    // Also works on inner scope. This will multiple the value of x only within the inner scope.
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // ...but won't carry that value on outside the brackets.
    println!("The value of x is: {x}");
}
