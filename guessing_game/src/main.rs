use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    // Access the local system provided number generator and generate a number within range
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    loop {
        println!("Input your guess");

        // Declare a new variable with inferred type
        let mut guess = String::new();

        // Read standard input and mutate into variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Eliminate whitespace `guess`, parse into u32 `parse`, handle wrong input
        // Match is essentially a switch
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Compare guess with secret number reference
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Break loop
            }
        }
    }
}
