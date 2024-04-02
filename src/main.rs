use attacks::Attacks;

use crate::perft::Perft;

mod attacks;
mod bit_ops;
mod bitboard;
mod board;
mod core_structs;
mod movegen;
mod perft;
mod rays;
mod zobrist;

fn main() {
    Attacks::initialize_slider_pieces();
    print!("Speed {} nps", Perft::test_speed());
}
