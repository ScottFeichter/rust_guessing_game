use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // let is const by default unless indicate mut for mutable

    io::stdin()
        .read_line(&mut guess) // & indicates that this is a reference - references are immutable by default so you must include the mut
        .expect("Failed to read line");

    println!("You guessed: {}", guess); // {} is a placeholder like ${} in js
}
