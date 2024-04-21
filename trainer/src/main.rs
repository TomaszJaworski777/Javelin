mod core_net_struct;
mod value_net;

use std::{process::Command, time::Instant};

use crate::value_net::ValueNet;
use colored::Colorize;
use datagen::{Files, PieceBoard};
use javelin::{Bitboard, Side, Square};
use rand::Rng;
use tch::nn::OptimizerConfig;
use rand::thread_rng;
use rand::seq::SliceRandom;
use tch::{Kind, Tensor};

fn main() {
    let value_net = ValueNet::new();
    let mut train_data = Files::new();
    let _ = train_data.load();

    println!("Loading dataset...");
    let data_set = prepare_value_dataset(train_data.value_data.clone());

    let mut learning_rate = 0.001;
    let mut optimizer = tch::nn::AdamW::default().build(&value_net.net.vs, learning_rate).unwrap();

    let timer = Instant::now();

    for epoch in 1..=2000000 {
        let mut total_loss = 0.0;
        let mut data_clone = data_set.clone();
        data_clone.shuffle(&mut thread_rng());
        let batches = prepare_batches(&data_clone);
        for (inputs, targets) in &batches {
            let outputs = value_net.net.evaluate(&inputs);
            let loss = (outputs - targets).pow_tensor_scalar(2).sum(Kind::Float).divide_scalar_(targets.numel() as f64);

            total_loss += loss.double_value(&[]) as f32;
            optimizer.backward_step(&loss);
        }

        value_net.save();
        value_net.export_final();
        create_snapshot(&value_net, epoch);

        if epoch % 7 == 0 && epoch != 0 {
            learning_rate *= 0.9;
            optimizer.set_lr(learning_rate);
        }

        println!("Epoch {}, time: {:.2}s, avg_loss: {}, lr: {}", 
            epoch,
            timer.elapsed().as_secs_f32(),
            total_loss / batches.len() as f32,
            learning_rate
        );
    }
}

#[allow(unused)]
fn create_snapshot(net: &ValueNet, index: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let snapshot_index = rng.gen_range(0, i32::MAX);
    net.export(format!("../../resources/training/snapshots/value_snapshot-{index}.net").as_str());
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
            result.push((extract_inputs(flip_board(&converted_bitboards)), 1.0 - result_score));
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
            result[piece_index * 64 + square.get_value()] = 1.0;
        }
    }
    result
}

fn flip_board(board: &[Bitboard; 12]) -> [Bitboard; 12] {
    let mut result = [Bitboard::EMPTY; 12];
    for piece_index in 0..6{
        result[piece_index] = board[piece_index+6].flip();
        result[piece_index+6] = board[piece_index].flip();
    }
    result
}

fn prepare_batches(data_set: &Vec<([f32; 768], f32)> ) -> Vec<(Tensor, Tensor)> {
    let mut result: Vec<(Tensor, Tensor)> = Vec::new();
    let mut batch_inputs: Vec<[f32; 768]> = Vec::new();
    let mut batch_outputs: Vec<[f32; 1]> = Vec::new();
    for (index, data_entry) in data_set.iter().enumerate(){
        if index != 0 && index % 16384 == 0 {
            let inputs_tensor = Tensor::from_slice2(&batch_inputs).to_kind(Kind::Float);
            let outputs_tensor = Tensor::from_slice2(&batch_outputs).to_kind(Kind::Float);
            result.push((inputs_tensor, outputs_tensor));
            batch_inputs.clear();
            batch_outputs.clear();
        }

        batch_inputs.push(data_entry.0);
        batch_outputs.push([data_entry.1]);
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

#[allow(unused)]
fn get_piece_tuple(board: &[Bitboard; 12], square: Square) -> (usize, Side) {
    for (index, bitboard) in board.iter().enumerate() {
        if !bitboard.get_bit(square) {
            continue;
        }
        let piece_index = (index % 6) + 1;
        let color = if index >= 6 { Side::BLACK } else { Side::WHITE };
        return (piece_index, color);
    }
    (0, Side::WHITE)
}

#[allow(unused)]
fn draw_board(board: &[Bitboard; 12]) {
    let piece_icons: [[&str; 7]; 2] =
    [[" . ", " P ", " N ", " B ", " R ", " Q ", " K "], [" . ", " p ", " n ", " b ", " r ", " q ", " k "]];
    let mut result = " ------------------------\n".to_string();
    for rank in (0..8).rev() {
        result += "|".to_string().as_str();
        for file in 0..8 {
            let square = Square::from_coords(rank, file);
            let piece_tuple = get_piece_tuple(&board, square);
            if piece_tuple.0 == 0 {
                result += piece_icons[0][usize::from(piece_tuple.0)];
            } else if piece_tuple.1 == Side::BLACK {
                result +=
                    piece_icons[Side::BLACK.current()][piece_tuple.0].blue().to_string().as_str();
            } else {
                result +=
                    piece_icons[Side::WHITE.current()][piece_tuple.0].yellow().to_string().as_str();
            }
        }
        result += "|".to_string().as_str();
        result += "\n".to_string().as_str();
    }
    result += " ------------------------\n".to_string().as_str();
    print!("{}", result);
}