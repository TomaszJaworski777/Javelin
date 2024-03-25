#![allow(dead_code)]

use colored::*;

pub struct Bitboard64{
    pub value: u64
}
impl Bitboard64 {
    pub const NULL: Bitboard64 = Bitboard64 {
        value: 0
    };

    #[inline]
    pub fn set_bit_to_one( &mut self, index: u8 ){
        self.value |= 1u64 << index;
    }

    #[inline]
    pub fn set_bit_to_zero( &mut self, index: u8 ){
        self.value &= !(1u64 << index);
    }

    #[inline]
    pub fn get_bit( &self, index: u8 ) -> u64 {
        self.value & (1u64 << index)
    }

    #[inline]
    pub fn set_bit_chunk( &mut self, index: u8, mask: u64, new_value: u64 ){
        self.value = (self.value & !(mask << index)) | new_value << index;
    }

    #[inline]
    pub fn get_bit_chunk( &self, index: u8, mask: u64 ) -> u64 {
        return (self.value & (mask << index)) >> index;
    }

    pub fn draw_bitboard( &self ){
        let mut result = " ------------------------\n".to_string();
        for rank in (0..8).rev() {
            result += format!("|").as_str();
            for file in 0..8{
                let square = rank * 8 + file;
                if self.get_bit( square ) == 0 { 
                    result += " 1 ".green().to_string().as_str();
                } 
                else 
                { 
                    result += " 0 ".red().to_string().as_str();
                }
            }
            result += format!("|\n").as_str();
        }
        result += format!(" ------------------------\n").as_str();
        result += format!("  Bitboard: {}\n", self.value).as_str();
        print!("{}\n", result);
    } 
}

pub struct Bitboard32{
    pub value: u32
}
impl Bitboard32 {
    pub const NULL: Bitboard32 = Bitboard32 {
        value: 0
    };
    
    #[inline]
    pub fn set_bit_to_one( &mut self, index: u8 ){
        self.value |= 1u32 << index;
    }

    #[inline]
    pub fn set_bit_to_zero( &mut self, index: u8 ){
        self.value &= !(1u32 << index);
    }

    #[inline]
    pub fn get_bit( &self, index: u8 ) -> u32 {
        self.value & (1u32 << index)
    }

    #[inline]
    pub fn set_bit_chunk( &mut self, index: u8, mask: u32, new_value: u32 ){
        self.value = (self.value & !(mask << index)) | new_value << index;
    }

    #[inline]
    pub fn get_bit_chunk( &self, index: u8, mask: u32 ) -> u32 {
        return (self.value & (mask << index)) >> index;
    }

    pub fn draw_bitboard( &self ){
        let mut result = " ------------------------\n".to_string();
        for rank in (0..4).rev() {
            result += format!("|").as_str();
            for file in 0..8{
                let square = rank * 8 + file;
                if self.get_bit( square ) == 0 { 
                    result += format!(" \x1b[32m1\x1b[0m ").as_str();
                } 
                else 
                { 
                    result += format!(" \x1b[31m0\x1b[0m ").as_str();
                }
            }
            result += format!("|\n").as_str();
        }
        result += format!(" ------------------------\n").as_str();
        result += format!("  Bitboard: {}\n", self.value).as_str();
        print!("{}", result);
    } 
}

pub struct Bitboard16{
    pub value: u16
}
impl Bitboard16 {
    pub const NULL: Bitboard16 = Bitboard16 {
        value: 0
    };

    #[inline]
    pub fn set_bit_to_one( &mut self, index: u8 ){
        self.value |= 1u16 << index;
    }

    #[inline]
    pub fn set_bit_to_zero( &mut self, index: u8 ){
        self.value &= !(1u16 << index);
    }

    #[inline]
    pub fn get_bit( &self, index: u8 ) -> u16 {
        self.value & (1u16 << index)
    }

    #[inline]
    pub fn set_bit_chunk( &mut self, index: u8, mask: u16, new_value: u16 ){
        self.value = (self.value & !(mask << index)) | new_value << index;
    }

    #[inline]
    pub fn get_bit_chunk( &self, index: u8, mask: u16 ) -> u16 {
        return (self.value & (mask << index)) >> index;
    }

    pub fn draw_bitboard( &self ){
        let mut result = " ------------------------\n".to_string();
        for rank in (0..4).rev() {
            result += format!("|").as_str();
            for file in 0..1{
                let square = rank * 8 + file;
                if self.get_bit( square ) == 0 { 
                    result += format!(" \x1b[32m1\x1b[0m ").as_str();
                } 
                else 
                { 
                    result += format!(" \x1b[31m0\x1b[0m ").as_str();
                }
            }
            result += format!("|\n").as_str();
        }
        result += format!(" ------------------------\n").as_str();
        result += format!("  Bitboard: {}\n", self.value).as_str();
        print!("{}", result);
    } 
}

pub struct Bitboard8{
    pub value: u8
}
impl Bitboard8 {
    pub const NULL: Bitboard8 = Bitboard8 {
        value: 0
    };

    #[inline]
    pub fn set_bit_to_one( &mut self, index: u8 ){
        self.value |= 1u8 << index;
    }

    #[inline]
    pub fn set_bit_to_zero( &mut self, index: u8 ){
        self.value &= !(1u8 << index);
    }

    #[inline]
    pub fn get_bit( &self, index: u8 ) -> u8 {
        self.value & (1u8 << index)
    }

    #[inline]
    pub fn set_bit_chunk( &mut self, index: u8, mask: u8, new_value: u8 ){
        self.value = (self.value & !(mask << index)) | new_value << index;
    }

    #[inline]
    pub fn get_bit_chunk( &self, index: u8, mask: u8 ) -> u8 {
        return (self.value & (mask << index)) >> index;
    }

    pub fn draw_bitboard( &self ){
        let mut result = " ------------------------\n".to_string();
        for rank in (0..4).rev() {
            result += format!("|").as_str();
            for file in 0..1{
                let square = rank * 8 + file;
                if self.get_bit( square ) == 0 { 
                    result += format!(" \x1b[32m1\x1b[0m ").as_str();
                } 
                else 
                { 
                    result += format!(" \x1b[31m0\x1b[0m ").as_str();
                }
            }
            result += format!("|\n").as_str();
        }
        result += format!(" ------------------------\n").as_str();
        result += format!("  Bitboard: {}\n", self.value).as_str();
        print!("{}", result);
    } 
}