#[allow(unused_imports)]
use std::env;
use std::{fs, path};
use std::io::{self, Write};
use std::process;

fn not_found(command: &str) {
    println!("{}: command not found", command.trim());
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn explain(proc_command: &str, builtins: [&str; 3], path_env: &str, token: Vec<&str>) {
    if token.len() != 2 {
        println!("type: expected 1 argument, got {}", token.len() - 1)
    }

    if builtins.contains(&proc_command) {
        println!("{} is a shell builtin", proc_command);
    } else {
        let split = &mut path_env.split(":");
        if let Some(path) = split.find(|path| std::fs::metadata(format!("{}/{}", path, proc_command)).is_ok()) {
            println!("{proc_command} is {path}/{proc_command}");
        } else {
            println!("{proc_command}: not found");
        }
    }
}

fn main() {
    let path_env = std::env::var("PATH").unwrap();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let builtins = ["exit", "echo", "type"];

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
            ["type", proc_command] => explain(proc_command, builtins, &path_env, token),
            _ => not_found(command),
        }
    }
}