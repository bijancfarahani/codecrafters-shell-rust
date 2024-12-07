use std::collections::HashSet;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::ExitCode;
// Type inference lets us omit an explicit type signature (which
// would be `HashSet<String>` in this example).
use anyhow::Error;

fn process_input(input: &String) -> Option<ExitCode> {
    let mut commands = HashSet::new();
    commands.insert("exit");
    commands.insert("echo");
    commands.insert("type");

    let tokens_orwhat: Vec<&str> = input.split_whitespace().collect();
    let command = tokens_orwhat[0];

    if command == "type" {
        if tokens_orwhat.len() == 1 {
            println!("$ ");
        } else if commands.contains(tokens_orwhat[1]) {
            println!("{} is a shell builtin", tokens_orwhat[1]);
        } else {
            println!("{}: not found", tokens_orwhat[1]);
        }
        None
    }
    match command {
        "exit" => Some(ExitCode::from(0)),

        "echo" => {
            for argument in tokens_orwhat.iter().skip(1) {
                print!("{}", argument);
                if argument != tokens_orwhat.last().unwrap() {
                    print!(" ");
                }
            }
            println!("");
            None
        }
        _ => {
            println!("{}: command not found", command);
            None
        }
    }
}

fn repl_loop() -> Option<ExitCode> {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    return process_input(&input);
}

fn main() -> Result<ExitCode, &'static str> {
    while true {
        let result = repl_loop();
        if result.is_some() {
            return Ok(result.unwrap());
        }
    }
    Err("Past infinite loop")
}
