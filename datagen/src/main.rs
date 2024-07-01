use crate::file_manager::Files;
use crate::selfplay_thread::SelfPlayThread;
use crate::structs::{ChessPolicyData, PieceBoard};
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod file_manager;
mod selfplay_thread;
mod structs;

struct GenData {
    files: Files,
    value_filtered: usize,
    policy_filtered: usize,
    games_played: u64,
    wins: u32,
    loses: u32,
    draws: u32,
    captures: u32,
    promotion: u32,
    under_promotions: u32,
    queen_castle: u32,
    king_castle: u32,
    en_passants: u32
}

fn main() {
    let gen_data = Arc::new(Mutex::new(GenData {
        files: Files::new(),
        value_filtered: 0,
        policy_filtered: 0,
        games_played: 0,
        wins: 0,
        loses: 0,
        draws: 0,
        captures: 0,
        promotion: 0,
        under_promotions: 0,
        queen_castle: 0,
        king_castle: 0,
        en_passants: 0
    }));

    let _ = gen_data.lock().unwrap().files.load();
    print_raport(&gen_data.lock().unwrap());

    let mut input = String::new();

    print!("Nodes per move: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Error reading input");
    let nodes_per_move: u16 = input.trim().parse().expect("Invalid number for nodes per move");
    input.clear();

    print!("Concurrency: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Error reading input");
    let concurrency: u8 = input.trim().parse().expect("Invalid number for concurrency");
    input.clear();

    for _ in 0..concurrency {
        let selfplay_thread = SelfPlayThread::new(gen_data.clone());
        selfplay_thread.run(nodes_per_move as u32);
    }

    let mut seconds = 0u128;
    loop {
        {
            let data = gen_data.lock().unwrap();
            print_raport(&data);
            println!("Positions per second: {:.1}", data.files.value_data.len() as f32 / seconds as f32);
            println!("Games per second: {:.1}\n", data.games_played as f32 / seconds as f32);
            println!("Games played: {}", data.games_played);
            println!("Games played: {}", data.games_played);
            println!("W/D/L: {}/{}/{}", data.wins, data.draws, data.loses);
            println!("Nodes per move: {}", nodes_per_move);
            println!("Concurrency: {}\n", concurrency);
            println!("Captures: {}", data.captures);
            println!("Promotions: {}", data.promotion);
            println!("Under Promotions: {}", data.under_promotions);
            println!("Castles (Q/K): {}/{}", data.queen_castle, data.king_castle);
            println!("En Passants: {}\n", data.en_passants);
            println!("Time until save: {}s", 1800 - (seconds % 1800));
            seconds += 1;

            if seconds % 1800 == 0 {
                let _ = data.files.save();
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn print_raport(data: &GenData) {
    clear_terminal_screen();
    println!("Welcome to selfplay data generator v{}\n", env!("CARGO_PKG_VERSION"));
    println!(
        "Value entries: {}({}B)",
        data.files.value_data.len(),
        number_scaler(data.files.value_data.len() * std::mem::size_of::<PieceBoard>())
    );
    println!(
        "Filtered: {}({}B)\n",
        data.value_filtered,
        number_scaler(data.value_filtered * std::mem::size_of::<PieceBoard>())
    );
    println!(
        "Policy entries: {}({}B)",
        data.files.policy_data.len(),
        number_scaler(data.files.policy_data.len() * std::mem::size_of::<ChessPolicyData>())
    );
    println!(
        "Filtered: {}({}B)\n",
        data.policy_filtered,
        number_scaler(data.policy_filtered * std::mem::size_of::<ChessPolicyData>())
    );
}

fn number_scaler(number: usize) -> String {
    const KILO: f32 = 1024.0;
    const MEGA: f32 = KILO * 1024.0;
    const GIGA: f32 = MEGA * 1024.0;

    if number < KILO as usize {
        number.to_string()
    } else if number < MEGA as usize {
        format!("{:.2}k", number as f32 / KILO)
    } else if number < GIGA as usize {
        format!("{:.2}M", number as f32 / MEGA)
    } else {
        format!("{:.2}G", number as f32 / GIGA)
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
