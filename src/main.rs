#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    print!("$ ");
    io::stdout().flush().unwrap();

    while stdin.read_line(&mut input).is_ok() {
        match input.trim().to_lowercase().as_str() {
            "exit" => break,
            "exit 0" => break,
            _ => println!("{}: command not found", input.trim()),
        }
        print!("$ ");
        io::stdout().flush().unwrap();
    };
}


