#![allow(dead_code)]

use crate::bitboards::Bitboard64;

mod board;
mod types;
mod bitboards;
mod consts;

fn main() {
    let mut board = board::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string());
    let x: u64 = board.pieces[0].get_bit(7);
    println!("Hello, world!");
    println!("{x}");
    let bitboard = Bitboard64 {
        value: 632423
    };
    bitboard.draw_bitboard();
    board.draw_board();
}