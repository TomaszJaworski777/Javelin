use attacks::Attacks;

mod attacks;
mod bit_ops;
mod bitboard;
mod board;
mod core_structs;
mod movegen;
mod rays;
mod zobrist;
mod perft;

fn main() {
    Attacks::initialize_slider_pieces();
}
