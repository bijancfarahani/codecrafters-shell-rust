#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::ExitCode;

use anyhow::Error;

fn process_input(input: &String) -> Option<ExitCode>{
    let tokens_orwhat: Vec<&str> = input.split_whitespace().collect();
    let command = tokens_orwhat[0];
    match command
    {
        "exit" => Some(ExitCode::from(0)),

        _ =>  None
    }
}

fn repl_loop() -> Option<ExitCode>{
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    return process_input(&input);
}

fn main()  -> Result<ExitCode, &'static str> {
    while true
    {
        let result = repl_loop();
        if result.is_some()
        {
            return Ok(result.unwrap());
        }
    }
    Err("Past infinite loop")
}
