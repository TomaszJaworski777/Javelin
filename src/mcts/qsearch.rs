use arrayvec::ArrayVec;
use spear::{ChessBoard, ChessPosition, Move};

use crate::{eval::Evaluation, see::SEE};

pub fn qsearch<'a, const STM_WHITE: bool, const NSTM_WHITE: bool>(
    position: &ChessPosition,
    mut alpha: i32,
    beta: i32,
    depth: u8,
) -> i32 {
    if position.board().is_insufficient_material()
        || position.is_repetition()
        || position.board().half_move_counter() >= 100
    {
        return 0;
    }

    let evaluation = Evaluation::evaluate::<STM_WHITE, NSTM_WHITE>(&position.board());

    if depth > 128 {
        return evaluation;
    }

    if evaluation >= beta {
        return beta;
    }

    if evaluation > alpha {
        alpha = evaluation;
    }

    let mut move_list = Vec::new();
    position.board().map_moves::<_, STM_WHITE, NSTM_WHITE>(|mv| move_list.push(mv));
    let mut move_orderer = MoveOrderer::new(&move_list);

    for _ in 0..move_list.len() {
        let mv = move_orderer.get_next_move(&position.board(), i32::max(1, alpha - evaluation - SEE::QS_MARGIN));
        if mv == Move::NULL {
            continue;
        }

        let mut board_copy = position.clone();
        board_copy.make_move::<STM_WHITE, NSTM_WHITE>(mv);

        let score = -qsearch::<NSTM_WHITE, STM_WHITE>(&board_copy, -beta, -alpha, depth + 1);

        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    return alpha;
}

struct MoveOrderer<'a> {
    move_list: &'a Vec<Move>,
    used_indecies: ArrayVec<Move, 256>,
}
impl<'a> MoveOrderer<'a> {
    fn new(move_list: &'a Vec<Move>) -> Self {
        Self { move_list, used_indecies: ArrayVec::new() }
    }

    fn get_next_move(&mut self, board: &ChessBoard, see_threshold: i32) -> Move {
        let mut best_move = Move::NULL;
        let mut best_score = i32::MIN;
        for mv in self.move_list {
            if self.used_indecies.contains(mv) {
                continue;
            }

            if !SEE::static_exchange_evaluation(&board, *mv, see_threshold) {
                self.used_indecies.push(*mv);
                continue;
            }

            let score = get_move_value(&board, mv);
            if score > best_score {
                best_score = score;
                best_move = *mv;
            }
        }
        self.used_indecies.push(best_move);
        best_move
    }
}

#[inline]
fn get_move_value(board: &ChessBoard, mv: &Move) -> i32 {
    let mut result = 0;

    if mv.is_capture() {
        let target_piece = board.get_piece_on_square(mv.get_to_square());
        let moving_piece = board.get_piece_on_square(mv.get_from_square());
        result += ((target_piece.get_raw() + 1) as i32 * 100) - (moving_piece.get_raw() + 1) as i32;
    }
    if mv.is_promotion() {
        result += ((mv.get_promotion_piece().get_raw() + 1) as i32) * 100;
    }

    return result;
}
