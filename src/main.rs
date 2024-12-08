use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::process::ExitCode;

fn lib_find_command() {}

fn execute_command_type(input: &str) {
    let commands: Vec<&str> = input.split_whitespace().collect();
    if commands.is_empty() {
        println!("$ ");
        return;
    }

    let shell_builtins = ["exit", "echo", "type"];
    let path_env = env::var("PATH").unwrap();
    let split = &mut path_env.split(':');
    for command in commands {
        if shell_builtins.contains(&command) {
            println!("{command} is a shell builtin");
        } else if let Some(path) =
            split.find(|path| std::fs::metadata(format!("{}/{}", path, command)).is_ok())
        {
            println!("{command} is {path}/{command}");
        } else {
            println!("{command}: not found");
        }
    }
}

fn execute_command_echo(input: &str) {
    println!("{}", input);
}

fn execute_command_exit() -> ExitCode {
    return ExitCode::SUCCESS;
}

fn execute_external_program(command: &str, arguments: &str) {
    let path_env = env::var("PATH").unwrap();
    let split = &mut path_env.split(':');
    if let Some(path) =
        split.find(|path| std::fs::metadata(format!("{}/{}", path, command)).is_ok())
    {
        let output = Command::new(command)
            .arg(arguments)
            .output()
            .expect("Failed to execute command");
        println!("{:?}", output.stdout);
    } else {
        println!("{}: Could not find in PATH", command);
    }
}

fn process_command(command: &str, arguments: &str) -> Option<ExitCode> {
    match command {
        "echo" => execute_command_echo(arguments),
        "exit" => return Some(execute_command_exit()),
        "type" => execute_command_type(arguments),
        _ => execute_external_program(&command, arguments),
        //_ => println!("{}: command not found", command),
    }
    None
}

fn process_input(input: &String) -> Option<ExitCode> {
    if input.is_empty() {
        return None;
    }

    let (command, remaining_input) = input.split_once(char::is_whitespace).unwrap();
    return process_command(command, remaining_input);
}

fn evalulate_user_input() -> Option<ExitCode> {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input.
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return process_input(&input);
}

fn main() -> Result<ExitCode, &'static str> {
    loop {
        let result = evalulate_user_input();
        if result.is_some() {
            return Ok(result.unwrap());
        }
    }
}
