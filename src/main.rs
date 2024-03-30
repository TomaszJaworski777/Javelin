#![allow(dead_code)]

use attacks::Attacks;
use board::{create_board, Board};

use std::time::Instant;

use crate::{bitboard::Bitboard, consts::Side, core_structs::Square};

mod attacks;
mod bit_ops;
mod bitboard;
mod board;
mod consts;
mod movegen;
mod core_structs;
mod zobrist;

fn main() {
    Attacks::initialize_slider_pieces();
    let board = create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    let start = Instant::now(); // Start timing

    benchmark_square_attack_mask(&board); // Function you want to measure

    let duration = start.elapsed(); // Calculate elapsed time

    println!("Time elapsed in benchmark_square_attack_mask() is: {:?}", duration);

    for square_index in 0..64 {
        if square_index % 8 == 0 {
            print!("\n");
        }
        print!(" {} ", if board.is_square_attacked(Square::from_raw(square_index), Side::WHITE) { "1" } else { "0" });
    }

    for square in Bitboard::ALL {
        Attacks::get_bishop_attacks_for_square(square, Bitboard::EMPTY).draw_bitboard();
    }
}

fn benchmark_square_attack_mask(board: &Board) {
    // Simulate some work for demonstration purposes
    for _ in 0..1_000_00 {
        for square_index in 0..64 {
            let result = board.is_square_attacked(Square::from_raw(square_index), Side::WHITE);
        }
    }
}
