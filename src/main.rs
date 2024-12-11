#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self, exit};

fn not_found(command: &str) {
    println!("{}: command not found", command.trim())
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split(" ").collect()
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command = input.trim();
        let token = tokenize(command);

        match token[..] {
            ["exit", code] => process::exit(code.parse::<i32>().unwrap()),
            ["echo"] => println!("{}", token[1..].join(" ")),
            _ => not_found(command),
        }
    }
}

/* Print $ and flush
print!("$ ");
io::stdout().flush().unwrap();

//Wait for user input and handle
let stdin = io::stdin();
let mut input = String::new();
stdin.read_line(&mut input).unwrap();

match input.trim().to_lowercase().as_str() {
    "exit" => break,
    "exit 0" => break,
    _ => println!("{}: command not found", input.trim()),
};
*/