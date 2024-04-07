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
    let board = create_board("3Q4/8/8/5K2/2k5/8/8/2r5 w - - 0 1");
    board.draw_board();

    let mut search = Search::new(&board);
    print!("{}\n", search.run().to_string());
}
