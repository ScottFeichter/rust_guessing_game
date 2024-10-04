use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // let is const by default unless indicate mut for mutable

        io::stdin()
            .read_line(&mut guess) // & indicates that this is a reference - references are immutable by default so you must include the mut
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // shadow variable
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}"); // {} is a placeholder like ${} in js

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
