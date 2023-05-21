use std::{io, cmp::Ordering};
use rand::{Rng, rngs::ThreadRng};

fn main() {
    println!("Guess the number!");

    let mut random_number_generator: ThreadRng = rand::thread_rng();
    let secret_number: u32 = random_number_generator.gen_range(1..=100);

    loop {
        println!("Please input your guess");
    
        let mut guess: String = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}