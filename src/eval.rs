mod pesto;
mod value_network;

use crate::core::{Board, Move, Side};

use self::pesto::Pesto;

pub use value_network::ValueNetwork;

pub const VALUE_NETWORK: ValueNetwork =
    unsafe { std::mem::transmute(*include_bytes!("../resources/nets/value-001.net")) };

pub struct Evaluation;
impl Evaluation {
    pub fn evaluate(board: &Board) -> i32 {
        let result = Pesto::get_score(&board);
        if board.side_to_move == Side::WHITE {
            result
        } else {
            -result
        }
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
