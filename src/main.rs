use std::io;

fn main() {
    println!("Hello to The Game Guessing the number!");
    println!("Please enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Hello {}", name.trim());
    println!("Please enter your guess:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("Your guess is {}", guess.trim());
    println!("");
}
