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
    let board = create_board("8/8/8/5K2/8/2kQ4/8/8 b - - 0 1");
    board.draw_board();

    let mut search = Search::new(&board);
}
