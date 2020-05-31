use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    // Game intro
    println!("Guess the number!");

    // Generate random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Remove for prod please
    // println!("The secret number is: {}", secret_number);

    // Loop over opotunities
    loop {
        
        // Ask for input
        println!("Please input your guess.");

        // Create guess mutable variable
        let mut guess = String::new();
    
        // Get variable from cli and set as guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
    
        // // Transform guess into a u32 variable
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // Create a match for error more granular error handling
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ask again for a number
        };
    
        // Show your guess
        println!("You guessed: {}", guess);
    
        // Match your answer
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

   
}