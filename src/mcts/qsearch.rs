use crate::{core::{Board, MoveList, MoveProvider}, eval::Evaluation};

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

    for mv in move_list {
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