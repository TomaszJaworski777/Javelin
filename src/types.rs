#![allow(dead_code)]

use crate::bitboards::Bitboard16;

pub struct Move{
    pub value: Bitboard16
}

pub struct Square{
    pub value: u8
}
impl Square {
    pub const NULL: Square = Square {
        value: 64
    };

    pub fn to_string( &self ) -> String{
        if self.value == 64 {
            return "NULL".to_string();
        }

        let file: u8 = self.value % 8;
        let rank: u8 = ((self.value as f32) / 8_f32).floor() as u8 + 1;
        return format!("{}{}", ('a' as u8 + file) as char, rank);
    }

    pub fn from_string( &mut self, square: &str ) {
        let signatures : Vec<char> = square.chars().collect();
        let file = signatures[0] as u8 - 'a' as u8;
        let rank = signatures[1].to_string().parse::<u8>().unwrap() - 1;
        self.value = rank * 8 + file;
    }
}