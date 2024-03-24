#![allow(dead_code)]

pub struct Bitboard64{
    pub value: u64
}
impl Bitboard64 {
    pub const NULL: Bitboard64 = Bitboard64 {
        value: 0
    };

    #[inline]
    pub fn set_bit( &mut self, index: u8 ){
        self.value |= 1 << index;
    }

    #[inline]
    pub fn get_bit( &self, index: u8 ) -> u64 {
        self.value & (1 << index)
    }

    #[inline]
    pub fn pop_bit( &self, index: u8 ) -> u64 {
        self.value & (1 << index)
    }

    pub fn draw_bitboard( &self ){
        let mut result = " ------------------------\n".to_string();
        for rank in (0..8).rev() {
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

pub struct Bitboard32{
    pub value: u32
}
impl Bitboard32 {
    pub const NULL: Bitboard32 = Bitboard32 {
        value: 0
    };
    
    #[inline]
    pub fn set_bit( &mut self, index: u8 ){
        self.value |= 1 << index;
    }

    #[inline]
    pub fn get_bit( &self, index: u8 ) -> u32 {
        self.value & (1 << index)
    }

    #[inline]
    pub fn pop_bit( &self, index: u8 ) -> u32 {
        self.value & (1 << index)
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
    pub fn set_bit( &mut self, index: u8 ){
        self.value |= 1 << index;
    }

    #[inline]
    pub fn get_bit( &self, index: u8 ) -> u16 {
        self.value & (1 << index)
    }

    #[inline]
    pub fn pop_bit( &self, index: u8 ) -> u16 {
        self.value & (1 << index)
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
    pub fn set_bit( &mut self, index: u8 ){
        self.value |= 1 << index;
    }

    #[inline]
    pub fn get_bit( &self, index: u8 ) -> u8 {
        self.value & (1 << index)
    }

    #[inline]
    pub fn pop_bit( &self, index: u8 ) -> u8 {
        self.value & (1 << index)
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