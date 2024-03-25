#![allow(dead_code)]

use std::fmt::Display;
use std::ops::{Shl, Shr, Not, BitOr, BitOrAssign, BitAnd, BitAndAssign, ShlAssign};
use std::cmp::PartialEq;
use colored::*;

pub struct Bitboard<T> {
    pub value: T,
}

// Implement generic methods for Bitboard.
impl<T> Bitboard<T> 
where
    T: Copy 
    + Shl<u8, Output = T> 
    + Shr<u8, Output = T> 
    + Not<Output = T> 
    + BitOr<Output = T> 
    + BitOrAssign 
    + BitAnd<Output = T> 
    + BitAndAssign
    + ShlAssign 
    + From<u8>
    + Display
    + PartialEq,
{
    pub fn new() -> Self {
        Bitboard { value: T::from(0u8) }
    }

    #[inline]
    pub fn set_bit_to_one(&mut self, index: u8) {
        self.value |= T::from(1u8) << index;
    }

    #[inline]
    pub fn set_bit_to_zero(&mut self, index: u8) {
        self.value &= !(T::from(1u8) << index);
    }

    #[inline]
    pub fn get_bit(&self, index: u8) -> T {
        self.value & (T::from(1u8) << index)
    }

    #[inline]
    pub fn set_bit_chunk(&mut self, index: u8, mask: T, new_value: T) {
        self.value = (self.value & !(mask << index)) | (new_value << index);
    }

    #[inline]
    pub fn get_bit_chunk(&self, index: u8, mask: T) -> T {
        (self.value & (mask << index)) >> index
    }

    pub fn draw_bitboard(&self) {
        let mut result = " ------------------------\n".to_string();
        for rank in (0..8).rev() {
            result += "|";
            for file in 0..8 {
                let square = rank * 8 + file;
                result += if self.get_bit(square as u8) != T::from(0u8) {
                    " 1 ".green()
                } else {
                    " 0 ".red()
                }.to_string().as_str();
            }
            result += "|\n";
        }
        result += " ------------------------\n";
        result += &format!("  Bitboard: {}\n", self.value);
        print!("{}\n", result);
    }
}