#![allow(dead_code)]

use crate::bitboards::{self, Bitboard16};

pub struct Move{
    pub value: Bitboard16
}

pub struct Square{
    pub value: u8
}
impl Square {
    pub const NULL: Square = Square {
        value: 0
    };
}