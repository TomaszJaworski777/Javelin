#![allow(dead_code)]

use crate::{attacks::Attacks, bit_ops::{get_bit, set_bit_to_one, set_bit_to_zero}, bitboard::Bitboard, consts::{CastleRights, Piece, Side}, types::Square, zobrist::ZobristKey};
use colored::*;

pub struct Board{
    pub pieces: [Bitboard; 6],
    pub piece_maps: [Bitboard; 2],
    pub castle_rights: u8,
    pub checkers: Bitboard,
    pub ortographic_pins: Bitboard,
    pub diagonal_pins: Bitboard,
    pub full_moves: u16,
    pub half_moves: u8,
    pub en_passant: Square,
    pub side_to_move: usize,
    pub zobrist: ZobristKey
}

impl Board {
    pub fn new() -> Self {
        Board{
            pieces: [Bitboard::EMPTY; 6],
            piece_maps: [Bitboard::EMPTY; 2],
            castle_rights: 0,
            checkers: Bitboard::EMPTY,
            ortographic_pins: Bitboard::EMPTY,
            diagonal_pins: Bitboard::EMPTY,
            full_moves: 0,
            half_moves: 0,
            en_passant: Square::NULL,
            side_to_move: 0,
            zobrist: ZobristKey::NULL
        }
    }

    #[inline]
    pub fn get_piece_mask( &self, piece: usize, side: usize ) -> Bitboard {
        self.pieces[piece-1].and(self.piece_maps[side])
    }
    
    #[inline]
    pub fn get_occupancy( &self ) -> Bitboard {
        self.piece_maps[0].or(self.piece_maps[1])
    }

    #[inline]
    pub fn get_piece_on_square( &self, square: Square ) -> (usize, usize){
        for piece_index in 0..6usize {
            if self.pieces[usize::from(piece_index)].get_bit(square) {
                return (piece_index + 1, self.get_piece_color_on_square(square));
            }
        }

        return (0,0);
    }

    #[inline]
    fn get_piece_color_on_square( &self, square: Square ) -> usize{
        if self.piece_maps[Side::WHITE].get_bit(square) {
            return Side::WHITE;
        }
        return Side::BLACK;
    }

    #[inline]
    pub fn set_piece_on_square( &mut self, square: Square, side: usize, piece: usize){
        self.pieces[piece-1].set_bit(square);
        self.piece_maps[side].set_bit(square);
        self.zobrist.update_piece_hash(piece, side, square)
    }

    #[inline]
    pub fn remove_piece_on_square( &mut self, square: Square, side: usize, piece: usize){
        self.pieces[piece-1].pop_bit(square);
        self.piece_maps[side].pop_bit(square);
        self.zobrist.update_piece_hash(piece, side, square)
    }

    #[inline]
    pub fn is_square_attacked( &self, square: Square, attacker_color: usize ) -> bool{
        let occupancy_mask = self.get_occupancy();
        if Attacks::get_bishop_attacks_for_square(square, occupancy_mask) & self.get_piece_mask(Piece::BISHOP, attacker_color).or(self.get_piece_mask(Piece::QUEEN, attacker_color)).get_value() > 0 { 
            return true;
        }
        if Attacks::get_knight_attacks_for_square(square) & self.get_piece_mask(Piece::KNIGHT, attacker_color).get_value() > 0 {
            return true;
        }
        if Attacks::get_rook_attacks_for_square(square, occupancy_mask) & self.get_piece_mask(Piece::ROOK, attacker_color).or(self.get_piece_mask(Piece::QUEEN, attacker_color)).get_value() > 0 {
            return true;
        }
        if Attacks::get_pawn_attacks_for_square(square, 1-attacker_color) & self.get_piece_mask(Piece::PAWN, attacker_color).get_value() > 0 {
            return true;
        }
        if Attacks::get_king_attacks_for_square(square) & self.get_piece_mask(Piece::KING, attacker_color).get_value() > 0 {
            return true;
        }
        false
    }

    pub fn draw_board( &self ){
        let piece_icons: [[&str; 7]; 2]= [[" . ", " P ", " N ", " B ", " R ", " Q ", " K "], [" . ", " p ", " n ", " b ", " r ", " q ", " k "]];

        let mut info = Vec::new();
        info.push("FEN: TBA");
        let zobrist = format!("Zobrist Key: {}", self.zobrist.key);
        info.push(zobrist.as_str());
        let mut castle_rights = "".to_string();
        if get_bit(self.castle_rights, CastleRights::WHITE_KING) > 0 {
            castle_rights += "K";
        }
        if get_bit(self.castle_rights, CastleRights::WHITE_QUEEN) > 0 {
            castle_rights += "Q";
        }
        if get_bit(self.castle_rights, CastleRights::BLACK_KING) > 0 {
            castle_rights += "k";
        }
        if get_bit(self.castle_rights, CastleRights::BLACK_QUEEN) > 0 {
            castle_rights += "q";
        }
        if castle_rights == "" {
            castle_rights = "-".to_string();
        }
        castle_rights = format!("Castle Rights: {}", castle_rights);
        info.push(castle_rights.as_str());
        let mut side_sign = "White".to_string();
        if self.side_to_move == Side::BLACK {
            side_sign = "Black".to_string();
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
                let square = Square::from_coords(rank, file);
                if square.equals(self.en_passant) {
                    result += " x ";
                    continue;
                }

                let piece_tuple = self.get_piece_on_square(square);
                if piece_tuple.1 == 2 {
                    result += piece_icons[0][usize::from(piece_tuple.0)];
                }
                else if piece_tuple.1 == Side::BLACK {
                    result += piece_icons[Side::BLACK][usize::from(piece_tuple.0)].blue().to_string().as_str();
                }
                else{
                    result += piece_icons[Side::WHITE][usize::from(piece_tuple.0)].yellow().to_string().as_str();
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
    let mut board = Board::new();
    let splits: Vec<&str> = fen.split_whitespace().collect();

    let ranks = splits[0].split('/');
    for (rank_index, rank) in ranks.enumerate() {
        let mut index = 0;
        let mut file = 0;
        while file < 8 {
            let square = Square::from_coords(7-rank_index, file);
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
                board.set_piece_on_square(square, side_to_move, Piece::PAWN)
            }
            else if piece_char == 'n' || piece_char == 'N' {
                board.set_piece_on_square(square, side_to_move, Piece::KNIGHT)
            }
            else if piece_char == 'b' || piece_char == 'B' {
                board.set_piece_on_square(square, side_to_move, Piece::BISHOP)
            }
            else if piece_char == 'r' || piece_char == 'R' {
                board.set_piece_on_square(square, side_to_move, Piece::ROOK)
            }
            else if piece_char == 'q' || piece_char == 'Q' {
                board.set_piece_on_square(square, side_to_move, Piece::QUEEN)
            }
            else if piece_char == 'k' || piece_char == 'K' {
                board.set_piece_on_square(square, side_to_move, Piece::KING)
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
        set_bit_to_one(&mut board.castle_rights, CastleRights::WHITE_KING);
        board.zobrist.update_castle_rights_hash(CastleRights::WHITE_KING as usize);
    }
    if splits[2].contains('Q') {
        set_bit_to_one(&mut board.castle_rights,CastleRights::WHITE_QUEEN);
        board.zobrist.update_castle_rights_hash(CastleRights::WHITE_QUEEN as usize);
    }
    if splits[2].contains('k') {
        set_bit_to_one(&mut board.castle_rights,CastleRights::BLACK_KING);
        board.zobrist.update_castle_rights_hash(CastleRights::BLACK_KING as usize);
    }
    if splits[2].contains('q') {
        set_bit_to_one(&mut board.castle_rights, CastleRights::BLACK_QUEEN);
        board.zobrist.update_castle_rights_hash(CastleRights::BLACK_QUEEN as usize);
    }

    board.en_passant = Square::NULL;
    if splits[3] != "-" {
        board.en_passant = Square::from_string(splits[3]);
        board.zobrist.update_en_passant_hash(board.en_passant);
    }

    board.full_moves = 0;
    board.half_moves = 0;

    if splits.len() > 4 {
        board.full_moves = splits[4].parse().unwrap();
    }

    if splits.len() > 5 {
        board.half_moves = splits[5].parse().unwrap();
    }

    board.checkers = Attacks::generate_checkers_mask(&board);
    board.ortographic_pins = Attacks::generate_ortographic_pins_mask(&board);
    board.diagonal_pins = Attacks::generate_diagonal_pins_mask(&board);

    return board;
}