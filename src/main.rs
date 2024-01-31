use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please, input a number of your choice:");

    let mut number_input: String = String::new();
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    let stdin_interface: io::Stdin = io::stdin();

    stdin_interface.read_line(&mut number_input)
        .expect("Failed to read input value");

    println!("You guessed {number_input}");
    println!("The correct number was {random_number}");
}
