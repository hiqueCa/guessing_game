use std::{io, u32};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please, input a number of your choice:");

    let mut number_input: String = String::new();
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    let stdin_interface: io::Stdin = io::stdin();

    stdin_interface.read_line(&mut number_input)
        .expect("Failed to read input value");

    let number_input: u32 = number_input
        .trim()
        .parse()
        .expect("Type a valid number.");

    println!("You guessed {number_input}");
    println!("The correct number was {random_number}");

    compare_numbers(number_input, random_number);
}

fn compare_numbers(guessed_number: u32, actual_number: u32) {
    match guessed_number.cmp(&actual_number) {
        Ordering::Equal => println!("You guessed the right number!"),
        Ordering::Greater => println!("You guessed a greater number."),
        Ordering::Less => println!("You guessed a smaller number."),
    };
}
