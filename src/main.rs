use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut loop_counter: i32 = 0;
    let random_number: i32 = rand::thread_rng().gen_range(1..=100);

    while loop_counter <= 3 {
        loop_counter = loop_counter + 1;

        game_loop(random_number);
    }
}

fn compare_numbers(guessed_number: i32, actual_number: i32) {
    match guessed_number.cmp(&actual_number) {
        Ordering::Equal => handle_correct_guess(),
        Ordering::Greater => handle_incorrect_guess("greater"),
        Ordering::Less => handle_incorrect_guess("lower"),
    };
}

fn handle_correct_guess() -> i32 {
    println!("You guessed the right number!");
    return 1;
}

fn handle_incorrect_guess(guess_type: &str) -> i32 {
    match guess_type {
        "greater" => {
            println!("You guessed a greater number.");
            return 0;
        },
        "lower" => {
            println!("You guessed a smaller number.");
            return 0;
        },
        &_ => todo!()
    }
}

fn game_loop(number_to_guess: i32) {
    println!("Please, input a number of your choice:");

    let mut number_input: String = String::new();
    let stdin_interface: io::Stdin = io::stdin();

    stdin_interface.read_line(&mut number_input)
        .expect("Failed to read input value");

    let number_input: i32 = number_input
        .trim()
        .parse()
        .expect("Type a valid number.");

    println!("You guessed {number_input}");

    compare_numbers(number_input, number_to_guess);
}
