use crate::{board::Board, core_structs::Side};

pub struct Evaluation;
impl Evaluation {
    pub fn evaluate( board: &Board ) -> f32 {
        let mut result = 0;
        for side in [Side::BLACK, Side::WHITE] {
            for piece_index in 0..6 {
                let piece_mask = board.get_piece_mask(piece_index + 1, side);
                result += piece_mask.pop_count() as i32 * (piece_index as i32 + 1) * (piece_index as i32 + 1) * 100
            }
            result = -result;
        }

        sigmoid(result) * board.side_to_move.multiplier() as f32
    }
}


pub fn sigmoid(input: i32) -> f32{
    2.0 / (1.0 + (-input as f32 / 400.0).exp()) - 1.0
}