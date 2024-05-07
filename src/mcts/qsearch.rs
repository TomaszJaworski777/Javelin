use arrayvec::ArrayVec;

use crate::{
    core::{Board, Move, MoveList, MoveProvider},
    eval::Evaluation,
    see::SEE,
};

pub fn qsearch<'a>(board: &Board, mut alpha: i32, beta: i32, depth: u8) -> i32 {
    if board.is_insufficient_material() || board.three_fold() || board.half_moves >= 100 {
        return 0;
    }

    let evaluation = Evaluation::evaluate(&board);

    if depth > 128 {
        return evaluation;
    }

    if evaluation >= beta {
        return beta;
    }

    if evaluation > alpha {
        alpha = evaluation;
    }

    let mut move_list = MoveList::new();
    MoveProvider::generate_moves::<true>(&mut move_list, &board);
    let mut move_orderer = MoveOrderer::new(&move_list);

    for _ in 0..move_list.len() {
        let mv = move_orderer.get_next_move(&board, i32::max(1, alpha - evaluation - SEE::QS_MARGIN));
        if mv == Move::NULL {
            continue;
        }

        let mut board_copy = board.clone();
        board_copy.make_move(mv);

        let score = -qsearch(&board_copy, -beta, -alpha, depth + 1);

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
    move_list: &'a MoveList,
    used_indecies: ArrayVec<Move, 256>,
}
impl<'a> MoveOrderer<'a> {
    fn new(move_list: &'a MoveList) -> Self {
        Self { move_list, used_indecies: ArrayVec::new() }
    }

    fn get_next_move(&mut self, board: &Board, see_threshold: i32) -> Move {
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

fn get_move_value(board: &Board, mv: &Move) -> i32 {
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
