use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please, input a number of your choice:");

    let mut number_input: String = String::new();
    let stdin_interface: io::Stdin = io::stdin();

    stdin_interface.read_line(&mut number_input)
        .expect("Failed to read input value");
}
