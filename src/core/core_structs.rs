use arrayvec::ArrayVec;
use std::ops::Add;
use std::ops::BitXor;

use crate::core::bit_ops::get_bit;
use crate::core::bit_ops::set_bit_to_one;
use crate::core::bit_ops::set_bit_to_zero;
use crate::core::{
    bit_ops::{get_bit_chunk, set_bit_chunk},
    bitboard::Bitboard,
};

pub type MoveList = ArrayVec<Move, 256>;

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    pub value: u16,
}
impl Move {
    pub const PROMOTION: u16 = 0b1000_000000_000000;
    pub const PROMOTION_KNIGHT_MASK: u16 = 0b1000_000000_000000;
    pub const PROMOTION_BISHOP_MASK: u16 = 0b1001_000000_000000;
    pub const PROMOTION_ROOK_MASK: u16 = 0b1010_000000_000000;
    pub const PROMOTION_QUEEN_MASK: u16 = 0b1011_000000_000000;
    pub const CAPTURE_MASK: u16 = 0b0100_000000_000000;
    pub const DOUBLE_PUSH_MASK: u16 = 0b0001_000000_000000;
    pub const KING_CASTLE_MASK: u16 = 0b0010_000000_000000;
    pub const QUEEN_CASTLE_MASK: u16 = 0b0011_000000_000000;
    pub const EN_PASSANT_MASK: u16 = 0b0001_000000_000000;
    pub const NULL: Self = Self { value: 0 };

    pub const PAWN_MOVES: [[Bitboard; 64]; 2] = {
        let mut result = [[Bitboard::EMPTY; 64]; 2];
        let mut side = 0;
        while side < 2 {
            let mut square_index = 0;
            while square_index < 64 {
                let square = Square::from_raw(square_index);
                let mut value = 0;
                if side == 0 {
                    value |= square.get_bit().shift_left(8).get_value();
                } else {
                    value |= square.get_bit().shift_right(8).get_value();
                }
                result[side][square_index] = Bitboard::from_raw(value);
                square_index += 1;
            }
            side += 1;
        }
        result
    };

    pub fn create_move(from_square: Square, to_square: Square, mask: u16) -> Self {
        Move { value: Move::init_move(from_square, to_square) | mask }
    }

    fn init_move(from_square: Square, to_square: Square) -> u16 {
        let mut result = 0u16;
        set_bit_chunk(&mut result, 0, 0b0000_000000_111111, from_square.get_value() as u16);
        set_bit_chunk(&mut result, 6, 0b0000_000000_111111, to_square.get_value() as u16);
        result
    }

    pub fn get_from_square(&self) -> Square {
        Square::from_raw(get_bit_chunk(self.value, 0, 0b0000_000000_111111) as usize)
    }

    pub fn get_to_square(&self) -> Square {
        Square::from_raw(get_bit_chunk(self.value, 6, 0b0000_000000_111111) as usize)
    }

    pub fn is_promotion(&self) -> bool {
        self.value & Move::PROMOTION > 0
    }

    pub fn get_promotion_piece(&self) -> usize {
        get_bit_chunk(self.value.into(), 12, 0b0000_000000_000011) + 2
    }

    pub fn is_capture(&self) -> bool {
        self.value & Move::CAPTURE_MASK > 0
    }

    pub fn is_en_passant(&self) -> bool {
        self.is_capture() && self.value & 0xF000 == Move::EN_PASSANT_MASK | Move::CAPTURE_MASK
    }

    pub fn is_double_push(&self) -> bool {
        self.value & 0xF000 == Move::DOUBLE_PUSH_MASK
    }

    pub fn is_king_castle(&self) -> bool {
        self.value & 0xF000 == Move::KING_CASTLE_MASK
    }

    pub fn is_queen_castle(&self) -> bool {
        self.value & 0xF000 == Move::QUEEN_CASTLE_MASK
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.get_from_square().to_string(),
            self.get_to_square().to_string(),
            if (self.value & Move::PROMOTION_KNIGHT_MASK) > 0 {
                ["n", "b", "r", "q"][self.get_promotion_piece() - 2]
            } else {
                ""
            }
        )
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Square {
    value: usize,
}
impl Square {
    pub const A1: Self = Self { value: 0 };
    pub const B1: Self = Self { value: 1 };
    pub const C1: Self = Self { value: 2 };
    pub const D1: Self = Self { value: 3 };
    pub const E1: Self = Self { value: 4 };
    pub const F1: Self = Self { value: 5 };
    pub const G1: Self = Self { value: 6 };
    pub const H1: Self = Self { value: 7 };
    pub const A2: Self = Self { value: 8 };
    pub const B2: Self = Self { value: 9 };
    pub const C2: Self = Self { value: 10 };
    pub const D2: Self = Self { value: 11 };
    pub const E2: Self = Self { value: 12 };
    pub const F2: Self = Self { value: 13 };
    pub const G2: Self = Self { value: 14 };
    pub const H2: Self = Self { value: 15 };
    pub const A3: Self = Self { value: 16 };
    pub const B3: Self = Self { value: 17 };
    pub const C3: Self = Self { value: 18 };
    pub const D3: Self = Self { value: 19 };
    pub const E3: Self = Self { value: 20 };
    pub const F3: Self = Self { value: 21 };
    pub const G3: Self = Self { value: 22 };
    pub const H3: Self = Self { value: 23 };
    pub const A4: Self = Self { value: 24 };
    pub const B4: Self = Self { value: 25 };
    pub const C4: Self = Self { value: 26 };
    pub const D4: Self = Self { value: 27 };
    pub const E4: Self = Self { value: 28 };
    pub const F4: Self = Self { value: 29 };
    pub const G4: Self = Self { value: 30 };
    pub const H4: Self = Self { value: 31 };
    pub const A5: Self = Self { value: 32 };
    pub const B5: Self = Self { value: 33 };
    pub const C5: Self = Self { value: 34 };
    pub const D5: Self = Self { value: 35 };
    pub const E5: Self = Self { value: 36 };
    pub const F5: Self = Self { value: 37 };
    pub const G5: Self = Self { value: 38 };
    pub const H5: Self = Self { value: 39 };
    pub const A6: Self = Self { value: 40 };
    pub const B6: Self = Self { value: 41 };
    pub const C6: Self = Self { value: 42 };
    pub const D6: Self = Self { value: 43 };
    pub const E6: Self = Self { value: 44 };
    pub const F6: Self = Self { value: 45 };
    pub const G6: Self = Self { value: 46 };
    pub const H6: Self = Self { value: 47 };
    pub const A7: Self = Self { value: 48 };
    pub const B7: Self = Self { value: 49 };
    pub const C7: Self = Self { value: 50 };
    pub const D7: Self = Self { value: 51 };
    pub const E7: Self = Self { value: 52 };
    pub const F7: Self = Self { value: 53 };
    pub const G7: Self = Self { value: 54 };
    pub const H7: Self = Self { value: 55 };
    pub const A8: Self = Self { value: 56 };
    pub const B8: Self = Self { value: 57 };
    pub const C8: Self = Self { value: 58 };
    pub const D8: Self = Self { value: 59 };
    pub const E8: Self = Self { value: 60 };
    pub const F8: Self = Self { value: 61 };
    pub const G8: Self = Self { value: 62 };
    pub const H8: Self = Self { value: 63 };
    pub const NULL: Self = Self { value: 64 };

    pub const fn from_raw(value: usize) -> Self {
        Self { value }
    }

    pub const fn from_coords(rank: usize, file: usize) -> Self {
        Self { value: rank * 8 + file }
    }

    pub const fn get_value(&self) -> usize {
        self.value
    }

    pub const fn get_rank(&self) -> usize {
        self.value / 8
    }

    pub const fn get_file(&self) -> usize {
        self.value % 8
    }

    pub const fn get_bit(&self) -> Bitboard {
        Bitboard::from_raw(1u64 << self.value)
    }

    pub const fn equals(&self, rhs: Square) -> bool {
        self.value == rhs.value
    }

    pub const fn flip(&self) -> Self {
        Self::from_raw(self.value ^ 56)
    }

    pub fn to_string(&self) -> String {
        if *self == Square::NULL {
            return "NULL".to_string();
        }

        let file: usize = self.value % 8;
        let rank: usize = ((self.value as f32) / 8_f32).floor() as usize + 1;
        return format!("{}{}", ('a' as usize + file) as u8 as char, rank);
    }

    pub fn from_string(square: &str) -> Square {
        let signatures: Vec<char> = square.chars().collect();
        let file = (signatures[0] as u8 - 'a' as u8).into();
        let rank = (signatures[1].to_string().parse::<u8>().unwrap() - 1).into();
        Square::from_coords(rank, file)
    }
}
impl Add<usize> for Square {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self { value: self.value + rhs }
    }
}
impl Add<Square> for usize {
    type Output = Square;

    fn add(self, rhs: Square) -> Self::Output {
        Square { value: self + rhs.get_value() }
    }
}
impl BitXor<usize> for Square {
    type Output = Self;

    fn bitxor(self, rhs: usize) -> Self::Output {
        Self::from_raw(self.value ^ rhs)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Side(usize);
impl Side {
    pub const WHITE: Side = Side::from_raw(0);
    pub const BLACK: Side = Side::from_raw(1);

    pub const fn from_raw(value: usize) -> Self {
        Self { 0: value }
    }

    pub const fn current(&self) -> usize {
        self.0
    }

    pub const fn opposite(&self) -> usize {
        1 - self.0
    }

    pub const fn flipped(&self) -> Self {
        Self { 0: 1 - self.0 }
    }

    pub fn mut_flip(&mut self) {
        self.0 = 1 - self.0;
    }
}

#[derive(Copy, Clone)]
pub struct CastleRights {
    value: u8,
}
impl CastleRights {
    pub const WHITE_KING: u8 = 0;
    pub const WHITE_QUEEN: u8 = 1;
    pub const BLACK_KING: u8 = 2;
    pub const BLACK_QUEEN: u8 = 3;
    pub const NULL: Self = Self { value: 0 };

    pub fn set_right(&mut self, right: u8) {
        set_bit_to_one(&mut self.value, right);
    }

    pub fn remove_right(&mut self, right: u8) {
        set_bit_to_zero(&mut self.value, right);
    }

    pub fn get_right(&self, right: u8) -> bool {
        get_bit(self.value, right) > 0
    }

    pub fn to_string(&self) -> String {
        let mut rights = "".to_string();
        if self.get_right(CastleRights::WHITE_KING) {
            rights += "K";
        }
        if self.get_right(CastleRights::WHITE_QUEEN) {
            rights += "Q";
        }
        if self.get_right(CastleRights::BLACK_KING) {
            rights += "k";
        }
        if self.get_right(CastleRights::BLACK_QUEEN) {
            rights += "q";
        }
        if rights == "" {
            rights = "-".to_string();
        }
        rights
    }
}

pub struct Piece;
impl Piece {
    pub const NONE: usize = 0;
    pub const PAWN: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const BISHOP: usize = 3;
    pub const ROOK: usize = 4;
    pub const QUEEN: usize = 5;
    pub const KING: usize = 6;
}
