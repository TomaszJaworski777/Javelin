#![allow(dead_code)]

use attacks::Attacks;
use bit_ops::Bitboard;
use board::create_board;

mod board;
mod types;
mod bit_ops;
mod consts;
mod zobrist;
mod attacks;

fn main() {
    create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").draw_board();
    for square_index in 0..64{
        let bb: u64 = Attacks::get_king_attacks_for_square( square_index );
        Bitboard::draw_bitboard(bb);
    }
}