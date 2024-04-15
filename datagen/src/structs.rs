use bytemuck::{Pod, Zeroable};
use javelin::{get_bit, Board};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct ChessMoveInfo {
    pub mov: u16,
    pub visits: u16,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct ChessPolicyData {
    pub board: PieceBoard,
    pub moves: [ChessMoveInfo; 104],
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct PieceBoard {
    pub piece_boards: [javelin::Bitboard; 4],
    pub score: f32,
    pub result: i8,
    pub side_to_move: u8,
    pub num: u8,
    pub extra: u8,
}

unsafe impl Zeroable for ChessMoveInfo {}
unsafe impl Pod for ChessMoveInfo {}

unsafe impl Zeroable for ChessPolicyData {}
unsafe impl Pod for ChessPolicyData {}

unsafe impl Zeroable for PieceBoard {}
unsafe impl Pod for PieceBoard {}

impl PieceBoard {
    pub fn from_board(board: &Board) -> Self {
        let mut piece_boards = [javelin::Bitboard::EMPTY; 4];

        for square in board.get_occupancy() {
            let (piece, color) = board.get_piece_on_square(square);
            for bit_index in 0..3usize {
                if get_bit(piece, bit_index as u8) > 0 {
                    piece_boards[bit_index].set_bit(square);
                }
                if color.current() == 1 {
                    piece_boards[3].set_bit(square);
                }
            }
        }

        Self { piece_boards, score: 0.0, result: 0, side_to_move: board.side_to_move.current() as u8, num: 0, extra: 0 }
    }
}
