use datagen::PieceBoard;
use javelin::{Bitboard, Square};
use tch::{Tensor, Kind};
use rand::thread_rng;
use rand::seq::SliceRandom;

#[allow(unused)]
pub struct ValueDataLoader;
#[allow(unused)]
impl ValueDataLoader {
    pub fn get_batches(data_set: &Vec<PieceBoard>, batch_size: usize) -> Vec<(Tensor, Tensor)> {
        let mut data = prepare_value_dataset(&data_set);
        data.shuffle(&mut thread_rng());

        let mut result: Vec<(Tensor, Tensor)> = Vec::new();
        let mut batch_inputs: Vec<[f32; 768]> = Vec::new();
        let mut batch_outputs: Vec<[f32; 1]> = Vec::new();
        for (index, data_entry) in data.iter().enumerate(){
            if index != 0 && index % batch_size == 0 {
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
}

fn prepare_value_dataset(data: &Vec<PieceBoard>) -> Vec<([f32; 768], f32)> {
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