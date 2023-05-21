fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Type required here as it can't implicitly guess type.
    let guess: u32 = "42".parse().expect("Not a number!");

     // addition
     let sum = 5 + 10;

     // subtraction
     let difference = 95.5 - 4.3;
 
     // multiplication
     let product = 4 * 30;
 
     // division
     let quotient = 56.7 / 32.2;
     let floored = 2 / 3; // Results in 0
 
     // remainder
     let remainder = 43 % 5;
}