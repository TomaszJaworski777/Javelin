#![allow(dead_code)]

use attacks::Attacks;
use board::{create_board, Board};

use crate::{
    consts::{Piece, Side},
    core_structs::Square,
};

mod attacks;
mod bit_ops;
mod bitboard;
mod board;
mod consts;
mod core_structs;
mod movegen;
mod zobrist;

fn main() {
    Attacks::initialize_slider_pieces();
    let board = create_board("8/8/3q4/6b1/8/8/r2K4/5n2 w - - 0 1");

    board.draw_board();
    board.checkers.draw_bitboard();
}
