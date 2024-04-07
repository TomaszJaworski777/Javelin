mod perft;
mod mcts;
mod eval;
mod core;

use mcts::Search;

use crate::core::{create_board, Attacks};

fn main() {
    Attacks::initialize_slider_pieces();
    let board = create_board("q7/8/8/2p5/1kp5/8/2KPP3/4RN2 w - - 0 1");
    board.draw_board();

    let mut search = Search::new(&board);
    print!("{}\n", search.run().to_string());
}
