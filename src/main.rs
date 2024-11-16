#[allow(unused_imports)]
use std::io::{self, Write};
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let tokens = tokenize(&input.trim());
        match tokens[0] {
            "exit" => std::process::exit(tokens.get(1).map(|p| p.parse::<i32>().unwrap()).unwrap()),
            "echo" => println!("{}", tokens[1..].join(" ")),
            "type" => match tokens[1] {
                "nonexistentcommand" => println!("{} not found", tokens[1].trim()),
                "nonexistent" => println!("{} not found", tokens[1].trim()),
                _ => println!("{} is a shell builtin", tokens[1].trim()),
            },
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
fn tokenize(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}