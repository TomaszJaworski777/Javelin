use crate::bit_ops::{get_bit_chunk, set_bit_chunk};

pub struct Move {
    value: u16
}
impl Move {
    pub const PROMOTION_KNIGHT_MASK: u16 = 0b1000_000000_000000;
    pub const PROMOTION_BISHOP_MASK: u16 = 0b1001_000000_000000;
    pub const PROMOTION_ROOK_MASK: u16   = 0b1010_000000_000000;
    pub const PROMOTION_QUEEN_MASK: u16  = 0b1011_000000_000000;
    pub const CAPTURE_MASK: u16          = 0b0100_000000_000000;
    pub const DOUBLE_PUSH_MASK: u16      = 0b0001_000000_000000;
    pub const KING_CASTLE_MASK: u16      = 0b0010_000000_000000;
    pub const QUEEN_CASTLE_MASK: u16     = 0b0011_000000_000000;
    pub const EN_PASSANT_MASK: u16       = 0b0001_000000_000000;

    pub const NULL: Move = Move {
        value: 0
    };

    #[inline]
    pub fn create_move( from_square: Square, to_square: Square, mask: u16  ) -> Self {
        Move {
            value: Move::init_move( from_square, to_square ) | mask
        }
    }

    #[inline]
    fn init_move( from_square: Square, to_square: Square ) -> u16 {
        let mut result = 0u16;
        set_bit_chunk(&mut result, 0, 0b0000_000000_111111, from_square.get_value() as u16);
        set_bit_chunk(&mut result, 6, 0b0000_000000_111111, to_square.get_value() as u16);
        result
    }

    #[inline]
    pub fn get_from_square(&self) -> Square {
        Square::from_raw(get_bit_chunk(self.value, 0, 0b0000_000000_111111) as usize)
    }

    #[inline]
    pub fn get_to_square(&self) -> Square {
        Square::from_raw(get_bit_chunk(self.value, 6, 0b0000_000000_111111) as usize)
    }

    #[inline]
    pub fn get_value(&self) -> u16 {
        self.value
    }
}

#[derive(Copy, Clone)]
pub struct Square{
    value: usize
}
impl Square {
    pub const NULL: Square = Square {
        value: 64
    };

    #[inline]
    pub const fn from_raw( value: usize ) -> Self {
        Self { value }
    }

    #[inline]
    pub const fn from_coords( rank: usize, file: usize ) -> Self {
        Self { 
            value: rank * 8 + file
        }
    }

    #[inline]
    pub const fn get_value(&self) -> usize {
        self.value
    }

    #[inline]
    pub const fn get_bit(&self) -> u64 {
        1u64 << self.value
    }

    #[inline]
    pub const fn equals(&self, rhs: Square) -> bool {
        self.value == rhs.get_value()
    }

    pub fn to_string( &self ) -> String{
        if self.value == 64 {
            return "NULL".to_string();
        }

        let file: usize = self.value % 8;
        let rank: usize = ((self.value as f32) / 8_f32).floor() as usize + 1;
        return format!("{}{}", ('a' as usize + file) as u8 as char, rank);
    }

    pub fn from_string( square: &str ) -> Square {
        let signatures : Vec<char> = square.chars().collect();
        let file = (signatures[0] as u8 - 'a' as u8).into();
        let rank = (signatures[1].to_string().parse::<u8>().unwrap() - 1).into();
        Square::from_coords(rank, file)
    }
}