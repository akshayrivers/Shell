use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    let stdin = io::stdin();

    // Define built-in commands
    let builtins = ["exit", "echo", "type", "cd", "pwd"];

    loop {
        prompt();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim().to_string();
        if input.is_empty() {
            continue;
        }
        handle_input(&input, &builtins);
        print!("\n");
    }
}

fn handle_input(input: &str, builtins: &[&str]) {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        return;
    }

    match tokens[0] {
        "exit" => {
            let code = tokens.get(1).and_then(|x| x.parse::<i32>().ok()).unwrap_or(0);
            std::process::exit(code);
        }
        "echo" => {
            let output = tokens[1..].join(" ");
            println!("{}", output);
        }
        "type" => {
            if tokens.len() > 1 {
                let command = tokens[1];
                if builtins.contains(&command) {
                    println!("{} is a custom shell builtin", command);
                } else {
                    println!("{} is a non-existent command", command);
                }
            } else {
                println!("Usage: type <command>");
            }
        }
        "cd" => {
            if tokens.len() > 1 {
                if let Err(e) = std::env::set_current_dir(tokens[1]) {
                    eprintln!("cd: {}: {}", tokens[1], e);
                }
            } else {
                println!("Usage: cd <directory>");
            }
        }
        "pwd" => {
            match std::env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("pwd: {}", e),
            }
        }
        _ => {
            execute_command(tokens);
        } // Try to execute the command
    }
}

fn execute_command(tokens: Vec<&str>) {
    let command = tokens[0];
    let args = &tokens[1..];

    let result = Command::new(command)
        .args(args)
        .stdout(Stdio::inherit()) // Forward stdout to the terminal
        .stderr(Stdio::inherit()) // Forward stderr to the terminal
        .stdin(Stdio::inherit())  // Forward stdin to the terminal
        .spawn();

    match result {
        Ok(mut child) => {
            // Wait for the command to finish
            child.wait().expect("Command failed to execute.");
        }
        Err(e) => {
            println!("{}: command not found ({})", command, e);
        }
    }
}

fn prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}
