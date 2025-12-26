use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number, punk!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Your guess, grasshopper!");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Next time, enter a number >:|");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Ha! Too small."),
            Ordering::Greater => println!("Too big this time."),
            Ordering::Equal => {
                println!("You cheated!");
                break;
            }
        }
    }
}
