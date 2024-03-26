#![allow(dead_code)]

use attacks::Attacks;
use bitboards::Bitboard;
use board::create_board;
use consts::{RankMask, Side};
use types::Square;

mod board;
mod types;
mod bitboards;
mod consts;
mod zobrist;
mod attacks;

fn main() {
    create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").draw_board();
    for square_index in 0..64{
        let bb: Bitboard<u64> = Attacks::get_knight_attack_for_square( square_index );
        bb.draw_bitboard();
    }
}