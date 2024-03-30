#![allow(dead_code)]

use attacks::Attacks;
use board::{create_board, Board};
use rays::Ray;

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
mod rays;
mod zobrist;

fn main() {
    Attacks::initialize_slider_pieces();
    let board = create_board("8/3q4/q5b1/1N6/2P1B1n1/r1QK3N/3n1n2/3r4 w - - 0 1");

    board.draw_board();
    board.checkers.draw_bitboard();
    board.ortographic_pins.draw_bitboard();
    board.diagonal_pins.draw_bitboard();
}
