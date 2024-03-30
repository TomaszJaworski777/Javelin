pub struct Side;
impl Side {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

pub struct Piece;
impl Piece {
    pub const NONE: usize = 0;
    pub const PAWN: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const BISHOP: usize = 3;
    pub const ROOK: usize = 4;
    pub const QUEEN: usize = 5;
    pub const KING: usize = 6;
}

pub struct CastleRights;
impl CastleRights {
    pub const BLACK_QUEEN: u8 = 0;
    pub const BLACK_KING: u8 = 1;
    pub const WHITE_QUEEN: u8 = 2;
    pub const WHITE_KING: u8 = 3;
}
