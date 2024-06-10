use colored::Colorize;
use datagen::ChessPolicyData;
use javelin::{Bitboard, Move, Side, Square};
use rand::seq::SliceRandom;
use rand::thread_rng;
use tch::{Kind, Tensor};

#[allow(unused)]
pub struct PolicyDataLoader;
#[allow(unused)]
impl PolicyDataLoader {
    pub fn get_batches(data_set: &Vec<ChessPolicyData>, batch_size: usize) -> Vec<(Tensor, Vec<Vec<(usize, usize)>>, Tensor)> {
        let mut data = prepare_policy_dataset(&data_set);
        data.shuffle(&mut thread_rng());

        let mut result: Vec<(Tensor, Vec<Vec<(usize, usize)>>, Tensor)> = Vec::new();
        let mut batch_inputs: Vec<[f32; 768]> = Vec::new();
        let mut batch_indecies: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut batch_outputs: Vec<[f32; 100]> = Vec::new();
        for (index, data_entry) in data.iter().enumerate() {
            if (index + 1) % batch_size == 0 {
                let inputs_tensor = Tensor::from_slice2(&batch_inputs).to_kind(Kind::Float);
                let outputs_tensor = Tensor::from_slice2(&batch_outputs).to_kind(Kind::Float);
                result.push((inputs_tensor, batch_indecies.clone(), outputs_tensor));
                batch_inputs.clear();
                batch_indecies.clear();
                batch_outputs.clear();
            }

            batch_inputs.push(data_entry.0);
            batch_indecies.push(data_entry.1.clone());
            batch_outputs.push(data_entry.2);
        }
        result
    } 
}

fn prepare_policy_dataset(data: &Vec<ChessPolicyData>) -> Vec<([f32; 768], Vec<(usize, usize)>, [f32; 100])> {
    let mut result: Vec<([f32; 768], Vec<(usize, usize)>, [f32; 100])> = Vec::new();
    for data_entry in data {
        if data_entry.board.num == 0 {
            continue;
        }

        let converted_bitboards = if data_entry.board.side_to_move == 0 {
            convert_to_12_bitboards(data_entry.board.piece_boards)
        } else {
            flip_board(&convert_to_12_bitboards(data_entry.board.piece_boards))
        };

        let mut total_visits = 0.0;
        let mut index_results: Vec<(usize, usize)> = Vec::new();
        let mut policy_results = [0.0; 100];
        for child_index in 0..data_entry.board.num as usize {
            let child = data_entry.moves[child_index];
            let mv = Move::from_raw(child.mv);
            let (from_index, to_index) = if data_entry.board.side_to_move == 0 {
                ( mv.get_from_square().get_value(), mv.get_to_square().get_value() )
            } else {
                ( mv.get_from_square().get_value() ^ 56, mv.get_to_square().get_value() ^ 56 )
            };
            index_results.push((from_index, to_index));
            policy_results[child_index] = child.visits as f32;
            total_visits += child.visits as f32;
        }

        for result_index in 0..data_entry.board.num as usize {
            policy_results[result_index] /= total_visits;
        }

        result.push((extract_inputs(converted_bitboards), index_results, policy_results));
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
        result[piece_index - 1].set_bit(square);
    }
    result
}

fn extract_inputs(board: [Bitboard; 12]) -> [f32; 768] {
    let mut result = [0.0; 768];
    for piece_index in 0..12 {
        for square in board[piece_index] {
            result[piece_index * 64 + square.get_value()] = 1.0;
        }
    }
    result
}

fn flip_board(board: &[Bitboard; 12]) -> [Bitboard; 12] {
    let mut result = [Bitboard::EMPTY; 12];
    for piece_index in 0..6 {
        result[piece_index] = board[piece_index + 6].flip();
        result[piece_index + 6] = board[piece_index].flip();
    }
    result
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
                result += piece_icons[Side::BLACK.current()][piece_tuple.0].blue().to_string().as_str();
            } else {
                result += piece_icons[Side::WHITE.current()][piece_tuple.0].yellow().to_string().as_str();
            }
        }
        result += "|".to_string().as_str();
        result += "\n".to_string().as_str();
    }
    result += " ------------------------\n".to_string().as_str();
    print!("{}", result);
}
