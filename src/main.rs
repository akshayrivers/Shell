#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    print!("$ ");
    io::stdout().flush().unwrap();
    
    while stdin.read_line(&mut input).is_ok() {
        let trimmed = input.trim();
        
        if trimmed == "exit" {
            break;
        }
        
        if !trimmed.is_empty() {
            println!("{}: command not found", trimmed);
        }
        
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
