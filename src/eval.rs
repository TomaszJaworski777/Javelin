mod pesto;
mod value_network;

use crate::core::{Board, Move, Side};
use crate::core::Bitboard;

pub use value_network::ValueNetwork;

pub const VALUE_NETWORK: ValueNetwork =
    unsafe { std::mem::transmute(*include_bytes!("../resources/nets/value-001.net")) };

pub struct Evaluation;
impl Evaluation {
    pub fn evaluate(board: &Board) -> f32 {
        let inputs = extract_inputs(board_to_flipped_12_bitboards(&board));
        let result = VALUE_NETWORK.evaluate(inputs);
        result
    }

    pub fn get_move_value(board: &Board, mv: Move) -> i32 {
        let mut result = 0;

        if mv.is_capture() {
            let (target_piece, _) = board.get_piece_on_square(mv.get_to_square());
            let (moving_piece, _) = board.get_piece_on_square(mv.get_from_square());
            result += (target_piece as i32 * 100) - moving_piece as i32;
        }
        if mv.is_promotion() {
            result += (mv.get_promotion_piece() as i32) * 100;
        }

        return result;
    }
}

fn board_to_flipped_12_bitboards(board: &Board) -> [Bitboard; 12] {
    let mut result = [Bitboard::EMPTY; 12];
    for piece_index in 0..6{
        if board.side_to_move == Side::WHITE {
            result[piece_index] = board.get_piece_mask(piece_index + 1, Side::WHITE);
            result[piece_index+6] = board.get_piece_mask(piece_index + 1, Side::BLACK);
        } else {
            result[piece_index+6] = board.get_piece_mask(piece_index + 1, Side::WHITE).flip();
            result[piece_index] = board.get_piece_mask(piece_index + 1, Side::BLACK).flip();
        }
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