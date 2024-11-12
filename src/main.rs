#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    
    loop {
        prompt();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input = input.trim().to_string();
        handle_input(&input);
    }
}
fn handle_input(input: &str) {
    let mut parts = input.split(' ');
    match parts.next().unwrap() {
        "exit" => std::process::exit(parts.next().and_then(|x| x.parse().ok()).unwrap_or(0)),
        "echo" => println!("{}", parts.collect::<Vec<&str>>().join(" ")),
        _ => println!("{input}: command not found"),
    }
}
fn prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}
