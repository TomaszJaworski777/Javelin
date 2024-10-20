mod benchmark;
mod commands;
mod core;
mod eval;
mod mcts;
mod neural;
mod options;
mod perft;
mod search_report;
mod see;

use benchmark::Benchmark;
use commands::Commands;
use std::{env, io::stdin, process::Command};

fn main() {
    let mut uci = Commands::new();

    let args: Vec<_> = env::args().collect();
    for (index, arg) in args.clone().into_iter().enumerate() {
        if arg == "bench" {
            Benchmark::run::<false>(if index < args.len() - 1 {
                args[index + 1].parse().unwrap_or_default()
            } else {
                5
            });
            return;
        }
    }

    println!("Javelin v{} by Tomasz Jaworski\n", env!("CARGO_PKG_VERSION"));

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
