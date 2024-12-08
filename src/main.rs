use std::env;

use std::io::{self, Write};

use std::path::PathBuf;

use std::process::Command;

fn find_exe(name: &str) -> Option<PathBuf> {
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let exe_path = path.join(name);

            if exe_path.is_file() {
                return Some(exe_path);
            }
        }
    }

    None
}

fn main() {
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        // Wait for user input

        let stdin = io::stdin();

        let mut input = String::new();

        stdin.read_line(&mut input).unwrap();

        let cmds: Vec<_> = input.split_whitespace().collect();

        if cmds.is_empty() {
            return;

            continue;
        }

        let cmd = cmds[0];

        let args = &cmds[1..];

        if BUILD_INS.contains(&cmd) {
            match cmd {
                "exit" => exit(args),

                "echo" => echo(args),

                "type" => cmd_type(args),

                _ => unreachable!(),
            };
        } else if let Some(path) = find_exe(cmd) {
            Command::new(path)
                .args(args)
                .status()
                .expect("failed to execute process");
        } else {
            println!("{}: command not found", cmd)
        }
    }
}
