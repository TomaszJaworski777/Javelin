use std::ops::{Index, IndexMut, Range};

use super::zobrist::ZobristKey;

#[derive(Clone, Copy, PartialEq)]
pub struct MoveHistory {
    values: [u64; 256],
    length: u8,
}

impl MoveHistory {
    pub fn new() -> Self {
        Self { values: [0; 256], length: 0 }
    }

    #[inline]
    pub fn push(&mut self, key: &ZobristKey) {
        let last_index = self.length;
        self[last_index] = key.key;
        self.length += 1;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.length = 0;
    }

    #[inline]
    pub fn range(&self) -> Range<u8> {
        0..self.length
    }
}

impl Index<u8> for MoveHistory {
    type Output = u64;

    fn index(&self, index: u8) -> &Self::Output {
        &self.values[index as usize]
    }
}

impl IndexMut<u8> for MoveHistory {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.values[index as usize]
    }
}
