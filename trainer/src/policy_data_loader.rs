use datagen::ChessPolicyData;
use javelin::{Bitboard, Move, Square};
use rand::thread_rng;
use rand::seq::SliceRandom;
use tch::{Tensor, Kind};

#[allow(unused)]
pub struct PolicyDataLoader{
    data: Vec<([f32; 768], [f32; 384])>
}
#[allow(unused)]
impl PolicyDataLoader {
    pub fn new( data: &Vec<ChessPolicyData> ) -> Self {
        Self { data: prepare_value_dataset(&data) }
    }

    pub fn get_batches(&self, batch_size: usize) -> Vec<(Tensor, Tensor, Tensor)> {
        let mut result: Vec<(Tensor, Tensor, Tensor)> = Vec::new();
        let mut batch_inputs: Vec<[f32; 768]> = Vec::new();
        let mut batch_outputs: Vec<[f32; 384]> = Vec::new();
        let mut batch_mask: Vec<[f32; 384]> = Vec::new();
        let mut data_clone: Vec<([f32; 768], [f32; 384])> = self.data.clone();
        data_clone.shuffle(&mut thread_rng());
        for (index, data_entry) in data_clone.iter().enumerate(){
            if index != 0 && index % batch_size == 0 {
                let inputs_tensor = Tensor::from_slice2(&batch_inputs).to_kind(Kind::Float);
                let outputs_tensor = Tensor::from_slice2(&batch_outputs).to_kind(Kind::Float);
                let mask_tensor = Tensor::from_slice2(&batch_mask).to_kind(Kind::Float);
                result.push((inputs_tensor, outputs_tensor, mask_tensor));
                batch_inputs.clear();
                batch_outputs.clear();
            }
    
            batch_inputs.push(data_entry.0);
            batch_outputs.push(data_entry.1);

            let mut mask = [0.0; 384];
            for output_index in 0..data_entry.1.len() {
                if data_entry.1[output_index] != 0.0 {
                    mask[output_index] = 1.0;
                }
            }

            batch_mask.push(mask);
        }
        result
    }
}

fn prepare_value_dataset(data: &Vec<ChessPolicyData>) -> Vec<([f32; 768], [f32; 384])> {
    let mut result: Vec<([f32; 768], [f32; 384])> = Vec::new();
    for data_entry in data {
        if data_entry.board.num == 0 {
            continue;
        }

        let converted_bitboards = convert_to_12_bitboards(data_entry.board.piece_boards);
        let mut result_score = [0.0; 384];

        let mut sum = 0.0;
        for child_index in 0..data_entry.board.num {
            let child = data_entry.moves[child_index as usize];
            let mv = Move::from_raw(child.mv);
            let index = if data_entry.board.side_to_move == 0 { mv.get_from_square().get_value() } else { mv.get_from_square().get_value() ^ 56 };
            let exp = (child.visits as f32).exp();
            result_score[index] = exp;
            sum += exp;
        }

        for i in 0..result_score.len(){
            result_score[i] /= sum;
        }

        if data_entry.board.side_to_move == 0 {
            result.push((extract_inputs(converted_bitboards), result_score));
        } else {
            result.push((extract_inputs(flip_board(&converted_bitboards)), result_score));
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