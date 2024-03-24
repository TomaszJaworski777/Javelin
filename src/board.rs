#![allow(dead_code)]

use crate::{bitboards::{Bitboard64, Bitboard8}, consts::Side, types::Square};

pub struct Board{
    pub pieces: [Bitboard64; 6],
    pub piece_tables: [Bitboard64; 2],
    pub castle_rights: Bitboard8,
    pub checkers: Bitboard64,
    pub full_moves: u16,
    pub half_moves: u8,
    pub en_passant: Square,
    pub side_to_move: u8,
    pub zobrist: u64
}

impl Board {
    pub const NULL: Board = Board{
        pieces: [Bitboard64::NULL; 6],
        piece_tables: [Bitboard64::NULL; 2],
        castle_rights: Bitboard8::NULL,
        checkers: Bitboard64::NULL,
        full_moves: 0,
        half_moves: 0,
        en_passant: Square::NULL,
        side_to_move: 0,
        zobrist: 0
    };

    pub fn get_piece_on_index( &self, index: u8 ) -> u8{
        for pieceIndex in 1..7u8 {
            if( self.pieces[usize::from(pieceIndex)].get_bit(index) > 0 ){
                return pieceIndex;
            }
        }

        return 0;
    }

    pub fn get__piece_color_on_index( &self, index: u8 ) -> u8{
        if( self.piece_tables[0].get_bit(index) > 0 ){
            return Side::WHITE;
        }
        else if( self.piece_tables[1].get_bit(index) > 0 ){
            return Side::BLACK;
        }
        return 2;
    }

    pub fn draw_board( &self ){

    }
}

pub fn create_board( fen: String ) -> Board {
    let mut result = Board::NULL;
    return result;
}