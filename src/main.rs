mod attacks;
mod bit_ops;
mod bitboard;
mod board;
mod core_structs;
mod movegen;
mod perft;
mod rays;
mod zobrist;
mod mcts;
mod eval;

use attacks::Attacks;
use board::create_board;
use mcts::Search;

fn main() {
    Attacks::initialize_slider_pieces();
    let board = create_board("q7/8/8/2p5/1kp5/8/2KPP3/4RN2 w - - 0 1");
    board.draw_board();

    let mut search = Search::new(&board);
    print!("{}\n", search.run().to_string());
}
