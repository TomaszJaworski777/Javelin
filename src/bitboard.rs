use colored::*;

use crate::types::Square;

#[derive(Copy, Clone)]
pub struct Bitboard{
    value: u64
}
impl Bitboard {
    pub const RANK_1: Self = Self::from_raw(0x00000000000000ff);
    pub const RANK_2: Self = Self::from_raw(0x000000000000ff00);
    pub const RANK_3: Self = Self::from_raw(0x0000000000ff0000);
    pub const RANK_4: Self = Self::from_raw(0x00000000ff000000);
    pub const RANK_5: Self = Self::from_raw(0x000000ff00000000);
    pub const RANK_6: Self = Self::from_raw(0x0000ff0000000000);
    pub const RANK_7: Self = Self::from_raw(0x00ff000000000000);
    pub const RANK_8: Self = Self::from_raw(0xff00000000000000);

    pub const FILE_A: Self = Self::from_raw(0x0101010101010101);
    pub const FILE_B: Self = Self::from_raw(0x0202020202020202);
    pub const FILE_C: Self = Self::from_raw(0x0404040404040404);
    pub const FILE_D: Self = Self::from_raw(0x0808080808080808);
    pub const FILE_E: Self = Self::from_raw(0x1010101010101010);
    pub const FILE_F: Self = Self::from_raw(0x2020202020202020);
    pub const FILE_G: Self = Self::from_raw(0x4040404040404040);
    pub const FILE_H: Self = Self::from_raw(0x8080808080808080);

    pub const ALL: Self = Self::from_raw(0xffffffffffffffff);
    pub const EMPTY: Self = Self::from_raw(0);

    pub const fn from_raw(value: u64) -> Self {
        Self { value }
    }

    pub const fn get_value(&self) -> u64{
        self.value
    }

    pub const fn pop_count(&self) -> u32{
        self.value.count_ones()
    }

    pub const fn ls1b_square(&self) -> Square{
        Square::from_raw(self.value.trailing_zeros() as usize)
    }

    pub fn set_bit(&mut self, square: Square){
        self.value |= square.get_bit()
    }

    pub fn pop_bit(&mut self, square: Square){
        self.value &= !square.get_bit()
    }

    pub fn pop_ls1b(&mut self) -> Square{
        let square = self.ls1b_square();
        self.value &= self.value - 1;
        square
    }

    pub const fn get_bit(&self, square: Square) -> bool{
        !self.and(Bitboard::from_raw(square.get_bit())).is_empty()
    }

    pub const fn is_empty(&self) -> bool {
        self.value == 0
    }

    pub const fn only_one_bit(&self) -> bool {
        !self.is_empty() && (self.value & self.value.wrapping_sub(1)) == 0
    }

    pub const fn multiple_one_bits(&self) -> bool {
        !self.is_empty() && (self.value & self.value.wrapping_sub(1)) != 0
    }

    pub const fn and(&self, rhs: Bitboard) -> Self {
        Self {
            value: self.value & rhs.value
        }
    }

    pub const fn or(&self, rhs: Bitboard) -> Self {
        Self {
            value: self.value | rhs.value
        }
    }

    pub const fn xor(&self, rhs: Bitboard) -> Self {
        Self {
            value: self.value ^ rhs.value
        }
    }

    pub const fn inverse(&self) -> Self {
        Self {
            value: !self.value
        }
    }

    pub const fn flip(&self) -> Self {
        Self {
            value: self.value.swap_bytes()
        }
    }

    pub const fn include(&self, square: Square) -> Self {
        Self {
            value: self.value | square.get_bit()
        }
    }

    pub const fn exclude(&self, square: Square) -> Self {
        Self {
            value: self.value & !square.get_bit()
        }
    }

    pub const fn shift_left(self, rhs: u32) -> Self {
        Self {
            value: self.value << rhs,
        }
    }

    pub const fn shift_right(self, rhs: u32) -> Self {
        Self {
            value: self.value >> rhs,
        }
    }

    pub fn draw_bitboard(&self) {
        let mut result = " ------------------------\n".to_string();
        for rank in (0..8).rev() {
            result += "|";
            for file in 0..8 {
                let square = Square::from_coords(rank, file);
                result += if self.get_bit(square) {
                    " 1 ".green()
                } else {
                    " 0 ".red()
                }.to_string().as_str();
            }
            result += "|\n";
        }
        result += " ------------------------\n";
        result += &format!("  Bitboard: {}\n", self.get_value());
        print!("{}\n", result);
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
            Some(self.board.pop_ls1b())
        }
    }
}