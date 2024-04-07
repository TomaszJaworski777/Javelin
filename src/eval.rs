use crate::core::{Board, Side};

pub struct Evaluation;
impl Evaluation {
    pub fn evaluate( board: &Board ) -> f32 {
        let mut result = 0;
        let values = [100, 300, 300, 500, 800];
        for side in [Side::BLACK, Side::WHITE] {
            result = -result;
            for piece_index in 0..5 {
                let piece_mask = board.get_piece_mask(piece_index + 1, side);
                result += piece_mask.pop_count() as i32 * values[piece_index];
            }
        }

        if board.side_to_move == Side::WHITE { sigmoid(result) } else { 1.0 - sigmoid(result) }
    }
}


pub fn sigmoid(input: i32) -> f32{
    1.0 / (1.0 + (-input as f32 / 400.0).exp())
}