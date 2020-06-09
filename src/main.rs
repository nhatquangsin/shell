mod command;
mod split;

use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};

use crate::command::process_cd;
use crate::split::split_command;

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut pre_command = None;

        while let Some(command) = commands.next() {
            let mut parts = split_command(command);
            let command = parts.swap_remove(0);
            let args = parts;

            match command.as_str() {
                "cd" => {
                    process_cd(args);
                    pre_command = None;
                }
                "exit" => return,
                command => {
                    let stdin = pre_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    if let Ok(child) = output {
                        pre_command = Some(child);
                    } else {
                        pre_command = None;
                        eprintln!("command not found: {}", command.to_string());
                    };
                }
            }
        }

        if let Some(mut final_command) = pre_command {
            final_command.wait().unwrap();
        }
    }
}
