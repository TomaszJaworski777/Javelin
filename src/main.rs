mod core;
mod eval;
mod mcts;
mod perft;

use mcts::Search;

use crate::{
    core::{create_board, Attacks},
    mcts::SearchRules,
};

fn main() {
    Attacks::initialize_slider_pieces();
    let board = create_board("4r1k1/p3qpbp/1p1p2n1/2pB2B1/7r/5Q2/PPP2P2/2KR2R1 b - - 3 25");
    board.draw_board();

    let mut rules = SearchRules::new();
    rules.time_for_move = 1000;

    let mut search = Search::new(&board);
    print!("{}\n", search.run(&rules).to_string());
}
