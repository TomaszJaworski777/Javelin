mod core_net_struct;
mod value_net;

use std::process::Command;

use crate::value_net::ValueNet;
use datagen::{Files, PieceBoard};
use javelin::{Bitboard, Square};
use rand::Rng;
use tch::nn::OptimizerConfig;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let mut value_net = ValueNet::new();
    let mut train_data = Files::new();
    let _ = train_data.load();

    let data_set = prepare_value_dataset(train_data.value_data);

    let learning_rate = 0.001;
    let mut optimizer = tch::nn::AdamW::default().build(&value_net.net.vs, learning_rate).unwrap();

    let mut raport_lines: Vec<String> = Vec::new(); 

    for epoch in 1..=200 {
        optimizer.zero_grad();
        let mut total_loss = 0.0;
        let mut data_clone = data_set.clone();
        data_clone.shuffle(&mut thread_rng());
        for (index, (inputs, target)) in data_clone.iter().enumerate() {
            let output = value_net.net.evaluate(&inputs.to_vec());
            let loss = output.subtract_scalar(*target as f64).pow_tensor_scalar(2);
            total_loss += loss.double_value(&[0]) as f32;

            loss.backward();
            optimizer.step();

            if index % 1000 == 0 {
                clear_terminal_screen();
                for line in &raport_lines{
                    println!("{line}");
                }
                println!("Current epoch progress: {}/{} ({:.2}%), curr_avg_loss: {}",
                    index,
                    data_set.len(),
                    (index as f32 / data_set.len() as f32) * 100.0,
                    total_loss / index as f32
                );
            }
        }

        value_net.save();

        raport_lines.push(format!("Epoch {}, avg_loss: {}", 
            epoch,
            total_loss / data_set.len() as f32
        ));
    }
}

fn create_snapshot(net: &ValueNet) -> i32 {
    let mut rng = rand::thread_rng();
    let snapshot_index = rng.gen_range(0, i32::MAX);
    net.export(format!("../resources/training/snapshots/value_snapshot-{snapshot_index}.net").as_str());
    snapshot_index
}

fn prepare_value_dataset(data: Vec<PieceBoard>) -> Vec<([f32; 768], f32)> {
    let mut result: Vec<([f32; 768], f32)> = Vec::new();
    for data_entry in data {
        if data_entry.score <= 0.0 || data_entry.score >= 1.0 {
            continue;
        }

        let converted_bitboards = convert_to_12_bitboards(data_entry.piece_boards);
        let result_score = (data_entry.result as f32 + 1.0)/2.0;
        if data_entry.side_to_move == 0 {
            result.push((extract_inputs(converted_bitboards), result_score));
        } else {
            result.push((extract_inputs(flip_board(converted_bitboards)), 1.0 - result_score));
        }
    }
    result
}

fn convert_to_12_bitboards(board: [Bitboard; 4]) -> [Bitboard; 12] {
    let mut result = [Bitboard::EMPTY; 12];
    for square_index in 0..64 {
        let square = Square::from_raw(square_index);
        let piece_index: usize = (if board[0].get_bit(square) { 1 } else { 0 }
            | if board[1].get_bit(square) { 2 } else { 0 }
            | if board[2].get_bit(square) { 4 } else { 0 })
            + if board[3].get_bit(square) { 6 } else { 0 };
        if piece_index == 0 {
            continue;
        } 
        result[piece_index-1].set_bit(square);
    }
    result
}

fn extract_inputs(board: [Bitboard; 12]) -> [f32; 768] {
    let mut result = [0.0; 768];
    for piece_index in 0..12{
        for square in board[piece_index]{
            result[square.get_value()] = 1.0;
        }
    }
    result
}

fn flip_board(board: [Bitboard; 12]) -> [Bitboard; 12] {
    let mut result = [Bitboard::EMPTY; 12];
    for piece_index in 0..12{
        for square in board[piece_index]{
            let target_square = square.flip();
            let target_index = if piece_index < 6 { piece_index + 6 } else { piece_index - 6 };
            result[target_index].set_bit(target_square);
        }
    }
    result
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