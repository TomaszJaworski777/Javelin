use crate::{Board, Move, Side};

const PRUNING_DEPTH: i32 = 10;
const QUIET_MARGIN: i32 = -64;
const NOISY_MARGIN: i32 = -20;
const PIECE_VALUES: [i32; 6] = [100, 300, 350, 500, 1000, 0];

pub fn static_exchange_evaluation(board: &Board, mv: Move, threshhold: i32) -> bool {
    let (next_victim, _) = if mv.is_promotion() {
        (mv.get_promotion_piece() + 1, Side::WHITE)
    } else {
        board.get_piece_on_square(mv.get_from_square())
    };

    false
}
