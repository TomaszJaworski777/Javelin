use std::ops::{Shl, Shr, Not, BitOr, BitOrAssign, BitAnd, BitAndAssign, ShlAssign};
use colored::*;

#[inline]
pub fn set_bit_to_one<T>(value: &mut T, index: u8)
where
    T: Copy
        + Shl<u8, Output = T>
        + BitOrAssign
        + From<u8>,
{
    *value |= T::from(1u8) << index;
}

#[inline]
pub fn set_bit_to_zero<T>(value: &mut T, index: u8)
where
    T: Copy
        + Shl<u8, Output = T>
        + Not<Output = T>
        + BitAndAssign
        + From<u8>,
{
    *value &= !(T::from(1u8) << index);
}

#[inline]
pub fn get_bit<T>(value: T, index: u8) -> T
where
    T: Copy
        + Shl<u8, Output = T>
        + BitAnd<Output = T>
        + From<u8>,
{
    value & (T::from(1u8) << index)
}

#[inline]
pub fn set_bit_chunk<T>(value: &mut T, index: u8, mask: T, new_value: T)
where
    T: Copy
        + Shl<u8, Output = T>
        + Not<Output = T>
        + BitOr<Output = T>
        + BitAnd<Output = T>
        + BitAndAssign
        + ShlAssign,
{
    *value = (*value & !(mask << index)) | (new_value << index);
}

#[inline]
pub fn get_bit_chunk<T>(value: T, index: u8, mask: T) -> T
where
    T: Copy
        + Shl<u8, Output = T>
        + Shr<u8, Output = T>
        + BitAnd<Output = T>,
{
    (value & (mask << index)) >> index
}

pub struct Bitboard;
impl Bitboard {
    pub fn draw_bitboard(value: u64) {
        let mut result = " ------------------------\n".to_string();
        for rank in (0..8).rev() {
            result += "|";
            for file in 0..8 {
                let square = rank * 8 + file;
                result += if get_bit::<u64>(value, square as u8) > 0 {
                    " 1 ".green()
                } else {
                    " 0 ".red()
                }.to_string().as_str();
            }
            result += "|\n";
        }
        result += " ------------------------\n";
        result += &format!("  Bitboard: {}\n", value);
        print!("{}\n", result);
    }
}