#![allow(dead_code)]

use attacks::{generate_bishop_attacks, generate_occupancy, mask_bishop_attacks, Attacks, BISHOP_OCCUPANCY_COUNT};
use bit_ops::{set_bit_to_one, Bitboard};
use board::create_board;
use types::Square;

use crate::attacks::{find_magic_number, test_magic_number, MAGIC_NUMBERS_BISHOP, ROOK_OCCUPANCY_COUNT};

mod board;
mod types;
mod bit_ops;
mod consts;
mod zobrist;
mod attacks;

fn main() {
    Attacks::initialize_slider_pieces();
    create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").draw_board();
    let mut occupancy = 0u64;
    set_bit_to_one(&mut occupancy, Square::create_from_string("e5").value);
    set_bit_to_one(&mut occupancy, Square::create_from_string("b4").value);
    set_bit_to_one(&mut occupancy, Square::create_from_string("g4").value);
    Bitboard::draw_bitboard(Attacks::get_rook_attacks_for_square(Square::create_from_string("e4").value as usize, occupancy));
}
