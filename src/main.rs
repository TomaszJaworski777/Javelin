mod benchmark;
mod commands;
mod core;
mod eval;
mod mcts;
mod options;
mod perft;
mod search_report;
mod see;
mod neural;

use commands::Commands;
use std::{io::stdin, process::Command};

fn main() {
    let mut uci = Commands::new();

    loop {
        let mut input_command = String::new();

        if stdin().read_line(&mut input_command).is_err() {
            println!("Error reading input, please try again.");
            continue;
        }

        let input_command = input_command.trim();

        let parts: Vec<&str> = input_command.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let args = &parts[1..].iter().map(|arg_str| arg_str.to_string()).collect::<Vec<String>>();

        if command == "exit" || command == "quit" {
            break;
        }

        if command == "clean" || command == "clear" || command == "cln" || command == "cls" {
            clear_terminal_screen();
            continue;
        }

        uci.execute_command(command, args);
    }
}

pub fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear").spawn().expect("clear command failed to start").wait().expect("failed to wait");
    };
}
