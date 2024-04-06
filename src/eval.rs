use crate::{board::Board, core_structs::Side};

pub struct Evaluation;
impl Evaluation {
    pub fn evaluate( board: &Board ) -> f32 {
        let mut result = 0;
        for side in [Side::BLACK, Side::WHITE] {
            for piece_index in 0..5 {
                let piece_mask = board.get_piece_mask(piece_index + 1, side);
                result += piece_mask.pop_count() as i32 * (piece_index as i32 + 1) * (piece_index as i32 + 1) * 100
            }
            result = -result;
        }

        if board.side_to_move == Side::WHITE { sigmoid(result) } else { 1.0 - sigmoid(result) }
    }
}


pub fn sigmoid(input: i32) -> f32{
    1.0 / (1.0 + (-input as f32 / 400.0).exp())
}