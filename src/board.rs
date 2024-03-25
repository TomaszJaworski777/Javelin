#![allow(dead_code)]

use crate::{bitboards::{Bitboard64, Bitboard8}, consts::{CastleRights, Piece, Side}, types::Square, zobrist::ZobristKey};
use colored::*;

pub struct Board{
    pub pieces: [Bitboard64; 6],
    pub piece_maps: [Bitboard64; 2],
    pub castle_rights: Bitboard8,
    pub checkers: Bitboard64,
    pub full_moves: u16,
    pub half_moves: u8,
    pub en_passant: Square,
    pub side_to_move: usize,
    pub zobrist: ZobristKey
}

impl Board {
    pub const NULL: Board = Board{
        pieces: [Bitboard64::NULL; 6],
        piece_maps: [Bitboard64::NULL; 2],
        castle_rights: Bitboard8::NULL,
        checkers: Bitboard64::NULL,
        full_moves: 0,
        half_moves: 0,
        en_passant: Square::NULL,
        side_to_move: 0,
        zobrist: ZobristKey::NULL
    };

    #[inline]
    pub fn get_piece_on_square( &self, square: u8 ) -> u8{
        for piece_index in 0..6u8 {
            if self.pieces[usize::from(piece_index)].get_bit(square) > 0 {
                return piece_index + 1;
            }
        }

        return 0;
    }

    #[inline]
    pub fn get_piece_color_on_square( &self, square: u8 ) -> usize{
        if self.piece_maps[0].get_bit(square) > 0 {
            return Side::WHITE;
        }
        else if self.piece_maps[1].get_bit(square) > 0 {
            return Side::BLACK;
        }
        return 2;
    }

    #[inline]
    pub fn set_piece_on_square( &mut self, square: u8, side: usize, piece: usize){
        self.pieces[piece-1].set_bit_to_one(square);
        self.piece_maps[side].set_bit_to_one(square);
        self.zobrist.update_piece_hash(piece, side, square as usize)
    }

    #[inline]
    pub fn remove_piece_on_square( &mut self, square: u8, side: usize, piece: usize){
        self.pieces[piece-1].set_bit_to_zero(square);
        self.piece_maps[side].set_bit_to_zero(square);
        self.zobrist.update_piece_hash(piece, side, square as usize)
    }

    pub fn draw_board( &self ){
        let piece_icons: [[&str; 7]; 2]= [[" . ", " P ", " N ", " B ", " R ", " Q ", " K "], [" . ", " p ", " n ", " b ", " r ", " q ", " k "]];

        let mut info = Vec::new();
        info.push("FEN: TBA");
        let zobrist = format!("Zobrist Key: {}", self.zobrist.key);
        info.push(zobrist.as_str());
        let mut castle_rights = "".to_string();
        if self.castle_rights.get_bit(CastleRights::WHITE_KING) > 0 {
            castle_rights += "K";
        }
        if self.castle_rights.get_bit(CastleRights::WHITE_QUEEN) > 0 {
            castle_rights += "Q";
        }
        if self.castle_rights.get_bit(CastleRights::BLACK_KING) > 0 {
            castle_rights += "k";
        }
        if self.castle_rights.get_bit(CastleRights::BLACK_QUEEN) > 0 {
            castle_rights += "q";
        }
        if castle_rights == "" {
            castle_rights = "-".to_string();
        }
        castle_rights = format!("Castle Rights: {}", castle_rights);
        info.push(castle_rights.as_str());
        let mut side_sign = "w".to_string();
        if self.side_to_move == Side::BLACK {
            side_sign = "b".to_string();
        }
        side_sign = format!("Side To Move: {}", side_sign);
        info.push(side_sign.as_str());
        let en_passant = format!( "En Passant: {}", self.en_passant.to_string() );
        info.push(en_passant.as_str());
        let moves = format!("Moves: {}", self.full_moves);
        info.push(moves.as_str());
        let half_moves = format!("Half Moves: {}", self.half_moves);
        info.push(half_moves.as_str());
        info.push("");

        let mut result = " ------------------------\n".to_string();
        for rank in (0..8).rev() {
            result += "|".to_string().as_str();
            for file in 0..8 {
                let square = rank * 8 + file;
                if square == self.en_passant.value {
                    result += " x ";
                    continue;
                }

                let piece = self.get_piece_on_square(square);
                let color = self.get_piece_color_on_square(square);
                if color == 2 {
                    result += piece_icons[0][usize::from(piece)];
                }
                else if color == Side::BLACK {
                    result += piece_icons[Side::BLACK][usize::from(piece)].blue().to_string().as_str();
                }
                else{
                    result += piece_icons[Side::WHITE][usize::from(piece)].yellow().to_string().as_str();
                }
            }
            result += format!("| {}", info[(7 - rank) as usize]).as_str();
            result += "\n".to_string().as_str();
        }
        result += " ------------------------\n".to_string().as_str();
        print!("{}\n", result);
    }
}

pub fn create_board( fen: &str ) -> Board {
    let mut board = Board::NULL;
    let splits: Vec<&str> = fen.split_whitespace().collect();

    let mut index: usize = 0;
    let mut file: usize = 0;
    let ranks = splits[0].split('/');
    for (rankIndex, rank) in ranks.enumerate() {
        index = 0;
        file = 0;
        while file < 8 {
            let square_index: u8 = ((7-rankIndex) * 8 + file) as u8;
            let piece_char = rank.as_bytes()[index] as char;
            if piece_char.is_numeric() {
                file += piece_char.to_string().parse::<usize>().unwrap();
                index += 1;
                continue;
            }

            let mut side_to_move = Side::WHITE;
            if piece_char > 'a' {
                side_to_move = Side::BLACK;
            }

            if piece_char == 'p' || piece_char == 'P' {
                board.set_piece_on_square(square_index, side_to_move, Piece::PAWN)
            }
            else if piece_char == 'n' || piece_char == 'N' {
                board.set_piece_on_square(square_index, side_to_move, Piece::KNIGHT)
            }
            else if piece_char == 'b' || piece_char == 'B' {
                board.set_piece_on_square(square_index, side_to_move, Piece::BISHOP)
            }
            else if piece_char == 'r' || piece_char == 'R' {
                board.set_piece_on_square(square_index, side_to_move, Piece::ROOK)
            }
            else if piece_char == 'q' || piece_char == 'Q' {
                board.set_piece_on_square(square_index, side_to_move, Piece::QUEEN)
            }
            else if piece_char == 'k' || piece_char == 'K' {
                board.set_piece_on_square(square_index, side_to_move, Piece::KING)
            }

            index += 1;
            file += 1;
        }
    }

    if splits[1] == "w" {
        board.side_to_move = Side::WHITE;
    }
    else {
        board.side_to_move = Side::BLACK;
        board.zobrist.update_side_to_move_hash();
    }

    if splits[2].contains('K') {
        board.castle_rights.set_bit_to_one(CastleRights::WHITE_KING);
        board.zobrist.update_castle_rights_hash(CastleRights::WHITE_KING as usize);
    }
    if splits[2].contains('Q') {
        board.castle_rights.set_bit_to_one(CastleRights::WHITE_QUEEN);
        board.zobrist.update_castle_rights_hash(CastleRights::WHITE_QUEEN as usize);
    }
    if splits[2].contains('k') {
        board.castle_rights.set_bit_to_one(CastleRights::BLACK_KING);
        board.zobrist.update_castle_rights_hash(CastleRights::BLACK_KING as usize);
    }
    if splits[2].contains('q') {
        board.castle_rights.set_bit_to_one(CastleRights::BLACK_QUEEN);
        board.zobrist.update_castle_rights_hash(CastleRights::BLACK_QUEEN as usize);
    }

    board.en_passant = Square::NULL;
    if splits[3] != "-" {
        board.en_passant.from_string(splits[3]);
        board.zobrist.update_en_passant_hash(board.en_passant.value);
    }

    board.full_moves = 0;
    board.half_moves = 0;

    if splits.len() > 4 {
        board.full_moves = splits[4].parse::<u16>().unwrap();
    }

    if splits.len() > 5 {
        board.half_moves = splits[5].parse::<u8>().unwrap();
    }

    return board;
}