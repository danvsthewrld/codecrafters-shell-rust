#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    print!("$ ");

    loop {
        
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        input.pop();
        println!("{}: command not found", input);
        println!("$ ");
    }
}
