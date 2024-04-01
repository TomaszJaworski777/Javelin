#![allow(dead_code)]

use attacks::Attacks;
use board::create_board;
use movegen::MoveProvider;
use std::time::Instant;

use crate::core_structs::MoveList;

mod attacks;
mod bit_ops;
mod bitboard;
mod board;
mod core_structs;
mod movegen;
mod rays;
mod zobrist;

fn main() {
    Attacks::initialize_slider_pieces();
    let board = create_board("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");

    board.draw_board();

    let mut moves = MoveList::new();
    let start = Instant::now();
    {
        for _ in 0..10_000 {
            moves = MoveList::new();
            MoveProvider::generate_moves(&mut moves, &board);
        }
    }
    let duration = start.elapsed();

    print!("{}\n", moves.len());
    for mov in moves {
        print!("{} - 1\n", mov.to_string());
        let mut new_board = board;
        new_board.make_move(mov);
        new_board.draw_board();
    }

    println!("Time elapsed is: {:?}", duration);
}
