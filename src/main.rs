#![allow(dead_code)]

use bitboards::Bitboard;
use board::create_board;

mod board;
mod types;
mod bitboards;
mod consts;
mod zobrist;

fn main() {
    create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").draw_board();
}