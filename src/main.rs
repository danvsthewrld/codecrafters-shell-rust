#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn not_found(command: &str) {
    println!("{}: command not found", command.trim());
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn explain(proc_command: &str) {
    match proc_command {
        "echo" | "exit" | "type" => println!("{} is a shell builtin", proc_command),
        _ => not_found(proc_command),
    }
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

        if token.is_empty() {
            // If no input is provided, just loop back to the prompt.
            continue;
        }

        match token[..] {
            ["exit", code] => {
                match code.parse::<i32>() {
                    Ok(exit_code) => process::exit(exit_code),
                    Err(_) => println!("exit: invalid exit code"),
                }
            }
            ["echo", ..] => println!("{}", token[1..].join(" ")),
            ["type", proc_command] => explain(proc_command),
            _ => not_found(command),
        }
    }
}