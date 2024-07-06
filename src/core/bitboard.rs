use crate::core::Square;
use colored::*;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr, ShrAssign};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Bitboard {
    value: u64,
}
impl Bitboard {
    pub const RANK_1: Self = Self::from_raw(0x00000000000000FF);
    pub const RANK_2: Self = Self::from_raw(0x000000000000FF00);
    pub const RANK_3: Self = Self::from_raw(0x0000000000FF0000);
    pub const RANK_4: Self = Self::from_raw(0x00000000FF000000);
    pub const RANK_5: Self = Self::from_raw(0x000000FF00000000);
    pub const RANK_6: Self = Self::from_raw(0x0000FF0000000000);
    pub const RANK_7: Self = Self::from_raw(0x00FF000000000000);
    pub const RANK_8: Self = Self::from_raw(0xFF00000000000000);

    pub const FILE_A: Self = Self::from_raw(0x0101010101010101);
    pub const FILE_B: Self = Self::from_raw(0x0202020202020202);
    pub const FILE_C: Self = Self::from_raw(0x0404040404040404);
    pub const FILE_D: Self = Self::from_raw(0x0808080808080808);
    pub const FILE_E: Self = Self::from_raw(0x1010101010101010);
    pub const FILE_F: Self = Self::from_raw(0x2020202020202020);
    pub const FILE_G: Self = Self::from_raw(0x4040404040404040);
    pub const FILE_H: Self = Self::from_raw(0x8080808080808080);

    pub const FULL: Self = Self::from_raw(0xFFFFFFFFFFFFFFFF);
    pub const EMPTY: Self = Self::from_raw(0);

    #[inline]
    pub const fn from_raw(value: u64) -> Self {
        Self { value }
    }

    #[inline]
    pub const fn get_value(&self) -> u64 {
        self.value
    }

    #[inline]
    pub const fn pop_count(&self) -> u32 {
        self.value.count_ones()
    }

    #[inline]
    pub const fn ls1b_square(&self) -> Square {
        Square::from_raw(self.value.trailing_zeros() as usize)
    }

    #[inline]
    pub fn set_bit(&mut self, square: Square) {
        self.mut_or(square.get_bit())
    }

    #[inline]
    pub fn pop_bit(&mut self, square: Square) {
        self.mut_and(square.get_bit().inverse())
    }

    #[inline]
    pub fn pop_ls1b_square(&mut self) -> Square {
        let square = self.ls1b_square();
        self.value &= self.value - 1;
        square
    }

    #[inline]
    pub const fn get_bit(&self, square: Square) -> bool {
        !self.and(square.get_bit()).is_empty()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.value == 0
    }

    #[inline]
    pub const fn is_not_empty(&self) -> bool {
        self.value != 0
    }

    #[inline]
    pub const fn equals(&self, rhs: Bitboard) -> bool {
        self.value == rhs.value
    }

    #[inline]
    pub const fn only_one_bit(&self) -> bool {
        !self.is_empty() && (self.value & self.value.wrapping_sub(1)) == 0
    }

    #[inline]
    pub const fn multiple_one_bits(&self) -> bool {
        !self.is_empty() && (self.value & self.value.wrapping_sub(1)) != 0
    }

    #[inline]
    pub fn mut_or(&mut self, rhs: Bitboard) {
        self.value |= rhs.get_value();
    }

    #[inline]
    pub fn mut_and(&mut self, rhs: Bitboard) {
        self.value &= rhs.get_value();
    }

    #[inline]
    pub const fn and(&self, rhs: Bitboard) -> Self {
        Self { value: self.value & rhs.value }
    }

    #[inline]
    pub const fn or(&self, rhs: Bitboard) -> Self {
        Self { value: self.value | rhs.value }
    }

    #[inline]
    pub const fn xor(&self, rhs: Bitboard) -> Self {
        Self { value: self.value ^ rhs.value }
    }

    #[inline]
    pub const fn inverse(&self) -> Self {
        Self { value: !self.value }
    }

    #[inline]
    pub const fn flip(&self) -> Self {
        Self { value: self.value.swap_bytes() }
    }

    #[inline]
    pub const fn include(&self, square: Square) -> Self {
        self.or(square.get_bit())
    }

    #[inline]
    pub const fn exclude(&self, square: Square) -> Self {
        self.and(square.get_bit().inverse())
    }

    #[inline]
    pub const fn shift_left(self, rhs: u32) -> Self {
        Self { value: self.value << rhs }
    }

    #[inline]
    pub const fn shift_right(self, rhs: u32) -> Self {
        Self { value: self.value >> rhs }
    }

    #[inline]
    pub const fn wrapping_mul(self, rhs: Bitboard) -> Self {
        Self { value: self.value.wrapping_mul(rhs.get_value()) }
    }

    pub fn draw_bitboard(&self) {
        let mut result = " ------------------------\n".to_string();
        for rank in (0..8).rev() {
            result += "|";
            for file in 0..8 {
                let square = Square::from_coords(rank, file);
                result += if self.get_bit(square) { " 1 ".green() } else { " 0 ".red() }.to_string().as_str();
            }
            result += "|\n";
        }
        result += " ------------------------\n";
        result += &format!("  Bitboard: {}\n", self.get_value());
        print!("{}\n", result);
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self::from_raw(value)
    }
}

impl From<Bitboard> for u64 {
    fn from(board: Bitboard) -> Self {
        board.get_value()
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.value & rhs.value)
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: u64) -> Self::Output {
        Self::from_raw(self.value & rhs)
    }
}

impl BitAnd<Bitboard> for u64 {
    type Output = Bitboard;

    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard::from_raw(self & rhs.value)
    }
}

impl BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.value &= rhs;
    }
}

impl BitAndAssign<Bitboard> for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.value &= rhs.get_value();
    }
}

impl BitAndAssign<Bitboard> for u64 {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        *self &= rhs.get_value();
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.value | rhs.value)
    }
}

impl BitOr<u64> for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: u64) -> Self::Output {
        Self::from_raw(self.value | rhs)
    }
}

impl BitOr<Bitboard> for u64 {
    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard::from_raw(self | rhs.value)
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.value |= rhs;
    }
}

impl BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.value |= rhs.get_value();
    }
}

impl BitOrAssign<Bitboard> for u64 {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        *self |= rhs.get_value();
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.value ^ rhs.value)
    }
}

impl BitXor<u64> for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: u64) -> Self::Output {
        Self::from_raw(self.value ^ rhs)
    }
}

impl BitXor<Bitboard> for u64 {
    type Output = Bitboard;

    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard::from_raw(self ^ rhs.value)
    }
}

impl BitXorAssign<u64> for Bitboard {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.value ^= rhs;
    }
}

impl BitXorAssign<Bitboard> for Bitboard {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.value ^= rhs.get_value();
    }
}

impl BitXorAssign<Bitboard> for u64 {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        *self ^= rhs.get_value();
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::from_raw(!self.value)
    }
}

impl Shl<u32> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Self::from_raw(self.value << rhs)
    }
}

impl ShlAssign<u32> for Bitboard {
    fn shl_assign(&mut self, rhs: u32) {
        self.value <<= rhs;
    }
}

impl Shr<u32> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Self::from_raw(self.value >> rhs)
    }
}

impl ShrAssign<u32> for Bitboard {
    fn shr_assign(&mut self, rhs: u32) {
        self.value >>= rhs;
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = Biterator;

    fn into_iter(self) -> Self::IntoIter {
        Biterator { board: self }
    }
}

pub struct Biterator {
    board: Bitboard,
}

impl Iterator for Biterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.board.is_empty() {
            None
        } else {
            Some(self.board.pop_ls1b_square())
        }
    }
}
