mod pesto;

use crate::core::{Board, Side};

use self::pesto::Pesto;

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
}
