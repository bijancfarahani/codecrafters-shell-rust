#[allow(unused_imports)]
use std::io::{self, Write};

fn repl_loop(){
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    println!("{}: command not found", input.trim());
}

fn main() {
    while true
    {
        repl_loop();
    }
}
