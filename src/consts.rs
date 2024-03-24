pub struct Side;
impl Side{
    pub const WHITE: u8 = 0;
    pub const BLACK: u8 = 1;
}

pub struct Piece;
impl Piece{
    pub const NONE: u8 = 0;
    pub const PAWN: u8 = 1;
    pub const KNIGHT: u8 = 2;
    pub const BISHOP: u8 = 3;
    pub const ROOK: u8 = 4;
    pub const QUEEN: u8 = 5;
    pub const KING: u8 = 6;
}