#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::ExitCode;
use std::{array, collections::HashSet};
// Type inference lets us omit an explicit type signature (which
// would be `HashSet<String>` in this example).
use anyhow::Error;
use std::env;

fn process_input(input: &String) -> Option<ExitCode> {
    let tokens_orwhat: Vec<&str> = input.split_whitespace().collect();
    let command = tokens_orwhat[0];

    match command {
        "type" => {
            // validate cmd args.
            if tokens_orwhat.len() == 1 {
                println!("$ ");
                return None;
            }
            let arg = tokens_orwhat[1];
            //let user_binaries = ["exit", "echo", "type"];
            let path_env = env::var("PATH").unwrap();
            let split = &mut path_env.split(':');
            if let Some(path) =
                split.find(|path| std::fs::metadata(format!("{}/{}", path, arg)).is_ok())
            {
                println!("{arg} is {path}/{arg}");
            } else {
                println!("{arg}: not found");
            }
            return None;
        }

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
