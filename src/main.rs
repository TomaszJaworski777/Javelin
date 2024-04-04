use attacks::Attacks;
use board::create_board;
use perft::Perft;

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

fn main() {
    Attacks::initialize_slider_pieces();
    let mut board = create_board("3r4/8/8/3k4/5b2/5P2/4RPP1/5BRK w - - 0 1");
    board.draw_board();

    Perft::perft_test();
}
