use arrayvec::ArrayVec;

use crate::{
    core::{Board, Move, MoveList, MoveProvider},
    eval::Evaluation,
};

pub fn qsearch(board: &Board, mut alpha: i32, beta: i32) -> i32 {
    let evaluation = Evaluation::evaluate(&board);

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
        let mv = move_orderer.get_next_move(&board);
        let mut board_copy = board.clone();
        board_copy.make_move(mv);

        let score = -qsearch(&board_copy, -beta, -alpha);

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

    fn get_next_move(&mut self, board: &Board) -> Move {
        let mut best_move = Move::NULL;
        let mut best_score = i32::MIN;
        for mv in self.move_list {
            if self.used_indecies.contains(mv) {
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
