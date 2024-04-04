use crate::{
    attacks::Attacks,
    bitboard::Bitboard,
    core_structs::{BaseRookPositions, CastleRights, Move, Piece, Side, Square, BASE_ROOK_POSITIONS},
    zobrist::ZobristKey,
};
use colored::*;

#[derive(Copy, Clone)]
pub struct Board {
    pieces: [Bitboard; 6],
    piece_maps: [Bitboard; 2],
    pub castle_rights: CastleRights,
    pub checkers: Bitboard,
    pub ortographic_pins: Bitboard,
    pub diagonal_pins: Bitboard,
    pub half_moves: u8,
    pub en_passant: Square,
    pub side_to_move: Side,
    pub zobrist: ZobristKey,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pieces: [Bitboard::EMPTY; 6],
            piece_maps: [Bitboard::EMPTY; 2],
            castle_rights: CastleRights::NULL,
            checkers: Bitboard::EMPTY,
            ortographic_pins: Bitboard::EMPTY,
            diagonal_pins: Bitboard::EMPTY,
            half_moves: 0,
            en_passant: Square::NULL,
            side_to_move: Side::WHITE,
            zobrist: ZobristKey::NULL,
        }
    }

    pub fn get_piece_mask(&self, piece: usize, side: Side) -> Bitboard {
        self.pieces[piece - 1] & self.piece_maps[side.current()]
    }

    pub fn get_occupancy(&self) -> Bitboard {
        self.piece_maps[0] | self.piece_maps[1]
    }

    pub fn get_allied_occupancy(&self) -> Bitboard {
        self.piece_maps[self.side_to_move.current()]
    }

    pub fn get_opponent_occupancy(&self) -> Bitboard {
        self.piece_maps[self.side_to_move.opposite()]
    }

    pub fn get_king_square(&self, color: Side) -> Square {
        self.get_piece_mask(Piece::KING, color).ls1b_square()
    }

    pub fn get_piece_on_square(&self, square: Square) -> (usize, Side) {
        for piece_index in 0..6usize {
            if self.pieces[usize::from(piece_index)].get_bit(square) {
                return (piece_index + 1, self.get_piece_color_on_square(square));
            }
        }

        return (0, Side::WHITE);
    }

    fn get_piece_color_on_square(&self, square: Square) -> Side {
        if self.piece_maps[Side::WHITE.current()].get_bit(square) {
            Side::WHITE
        } else {
            Side::BLACK
        }
    }

    pub fn set_piece_on_square(&mut self, square: Square, side: Side, piece: usize) {
        self.pieces[piece - 1].set_bit(square);
        self.piece_maps[side.current()].set_bit(square);
        self.zobrist.update_piece_hash(piece - 1, side.current(), square)
    }

    pub fn remove_piece_on_square(&mut self, square: Square, side: Side, piece: usize) {
        self.pieces[piece - 1].pop_bit(square);
        self.piece_maps[side.current()].pop_bit(square);
        self.zobrist.update_piece_hash(piece - 1, side.current(), square)
    }

    pub fn is_in_check(&self) -> bool {
        self.checkers.is_not_empty()
    }

    pub fn is_square_attacked_extended(&self, square: Square, attacker_color: Side, occupancy_mask: Bitboard) -> bool {
        let bishop_queen_mask =
            self.get_piece_mask(Piece::BISHOP, attacker_color) | self.get_piece_mask(Piece::QUEEN, attacker_color);
        let rook_queen_mask =
            self.get_piece_mask(Piece::ROOK, attacker_color) | self.get_piece_mask(Piece::QUEEN, attacker_color);

        if (Attacks::get_bishop_attacks_for_square(square, occupancy_mask) & bishop_queen_mask).is_not_empty()
            || (Attacks::get_knight_attacks_for_square(square) & self.get_piece_mask(Piece::KNIGHT, attacker_color))
                .is_not_empty()
            || (Attacks::get_rook_attacks_for_square(square, occupancy_mask) & rook_queen_mask).is_not_empty()
            || (Attacks::get_pawn_attacks_for_square(square, attacker_color.flipped())
                & self.get_piece_mask(Piece::PAWN, attacker_color))
            .is_not_empty()
            || (Attacks::get_king_attacks_for_square(square) & self.get_piece_mask(Piece::KING, attacker_color))
                .is_not_empty()
        {
            return true;
        }
        false
    }

    pub fn is_square_attacked(&self, square: Square, attacker_color: Side) -> bool {
        self.is_square_attacked_extended(square, attacker_color, self.get_occupancy())
    }

    pub fn any_squares_attacked(&self, squares: Bitboard, attacker_color: Side) -> bool {
        for square in squares {
            if self.is_square_attacked(square, attacker_color) {
                return true;
            }
        }
        return false;
    }

    pub fn make_move(&mut self, _move: Move) {
        let from_square = _move.get_from_square();
        let to_square = _move.get_to_square();
        let moving_piece = self.get_piece_on_square(from_square);
        let target_piece_square = if _move.is_en_passant() { to_square ^ 8 } else { to_square };
        let target_piece = self.get_piece_on_square(target_piece_square);
        let castle_rights_offset = (self.side_to_move.current() * 2) as u8;
        let square_value_offset = self.side_to_move.current() * 56;

        self.remove_piece_on_square(from_square, moving_piece.1, moving_piece.0);
        if target_piece.0 != Piece::NONE {
            self.remove_piece_on_square(target_piece_square, target_piece.1, target_piece.0);
        }

        let destination_piece = if _move.is_promotion() { _move.get_promotion_piece() } else { moving_piece.0 };
        self.set_piece_on_square(to_square, moving_piece.1, destination_piece);

        let remove_castle_rights = |board: &mut Board| {
            board.castle_rights.remove_right(CastleRights::WHITE_KING + castle_rights_offset);
            board.castle_rights.remove_right(CastleRights::WHITE_QUEEN + castle_rights_offset);
            board.zobrist.update_castle_rights_hash((CastleRights::WHITE_KING + castle_rights_offset) as usize);
            board.zobrist.update_castle_rights_hash((CastleRights::WHITE_QUEEN + castle_rights_offset) as usize);
        };

        if _move.is_king_castle() {
            let rook_position = BaseRookPositions::get_king_side() + square_value_offset;
            let rook_destination = Square::F1 + square_value_offset;
            self.remove_piece_on_square(rook_position, moving_piece.1, Piece::ROOK);
            self.set_piece_on_square(rook_destination, moving_piece.1, Piece::ROOK);

            remove_castle_rights(self);
        } else if _move.is_queen_castle() {
            let rook_position = BaseRookPositions::get_queen_side() + square_value_offset;
            let rook_destination = Square::D1 + square_value_offset;
            self.remove_piece_on_square(rook_position, moving_piece.1, Piece::ROOK);
            self.set_piece_on_square(rook_destination, moving_piece.1, Piece::ROOK);

            remove_castle_rights(self);
        }

        if moving_piece.0 == Piece::KING {
            remove_castle_rights(self);
        } else if moving_piece.0 == Piece::ROOK {
            let king_rook_position = BaseRookPositions::get_king_side() + square_value_offset;
            let queen_rook_position = BaseRookPositions::get_queen_side() + square_value_offset;

            if from_square == king_rook_position {
                self.castle_rights.remove_right(CastleRights::WHITE_KING + castle_rights_offset);
                self.zobrist.update_castle_rights_hash((CastleRights::WHITE_KING + castle_rights_offset) as usize);
            } else if from_square == queen_rook_position {
                self.castle_rights.remove_right(CastleRights::WHITE_QUEEN + castle_rights_offset);
                self.zobrist.update_castle_rights_hash((CastleRights::WHITE_QUEEN + castle_rights_offset) as usize);
            }
        }
        if target_piece.0 == Piece::ROOK {
            let king_rook_position = BaseRookPositions::get_king_side() + (self.side_to_move.opposite() * 56);
            let queen_rook_position = BaseRookPositions::get_queen_side() + (self.side_to_move.opposite() * 56);

            if to_square == king_rook_position {
                self.castle_rights.remove_right(CastleRights::WHITE_KING + (self.side_to_move.opposite() * 2) as u8);
                self.zobrist.update_castle_rights_hash(
                    (CastleRights::WHITE_KING + (self.side_to_move.opposite() * 2) as u8) as usize,
                );
            } else if to_square == queen_rook_position {
                self.castle_rights.remove_right(CastleRights::WHITE_QUEEN + (self.side_to_move.opposite() * 2) as u8);
                self.zobrist.update_castle_rights_hash(
                    (CastleRights::WHITE_QUEEN + (self.side_to_move.opposite() * 2) as u8) as usize,
                );
            }
        }

        if _move.is_double_push() {
            self.en_passant = from_square ^ 24;
            self.zobrist.update_en_passant_hash(self.en_passant);
        } else if self.en_passant != Square::NULL {
            self.zobrist.update_en_passant_hash(self.en_passant);
            self.en_passant = Square::NULL;
        }

        self.half_moves += 1;
        if _move.is_capture() || moving_piece.0 == Piece::PAWN {
            self.half_moves = 0;
        }

        self.side_to_move.mut_flip();
        self.zobrist.update_side_to_move_hash();

        self.checkers = Attacks::generate_checkers_mask(&self);
        self.ortographic_pins = Attacks::generate_ortographic_pins_mask(&self);
        self.diagonal_pins = Attacks::generate_diagonal_pins_mask(&self);
    }

    #[allow(dead_code)]
    pub fn draw_board(&self) {
        let piece_icons: [[&str; 7]; 2] =
            [[" . ", " P ", " N ", " B ", " R ", " Q ", " K "], [" . ", " p ", " n ", " b ", " r ", " q ", " k "]];

        let mut info = Vec::new();
        info.push("FEN: TBA");
        let zobrist = format!("Zobrist Key: {}", self.zobrist.key);
        info.push(zobrist.as_str());

        let castle_rights = format!("Castle Rights: {}", self.castle_rights.to_string());
        info.push(castle_rights.as_str());
        let mut side_sign = "White".to_string();
        if self.side_to_move == Side::BLACK {
            side_sign = "Black".to_string();
        }
        side_sign = format!("Side To Move: {}", side_sign);
        info.push(side_sign.as_str());
        let en_passant = format!("En Passant: {}", self.en_passant.to_string());
        info.push(en_passant.as_str());
        let half_moves = format!("Half Moves: {}", self.half_moves);
        info.push(half_moves.as_str());
        let in_check = format!("In Check: {}", self.is_in_check());
        info.push(in_check.as_str());
        info.push("");

        let mut result = " ------------------------\n".to_string();
        for rank in (0..8).rev() {
            result += "|".to_string().as_str();
            for file in 0..8 {
                let square = Square::from_coords(rank, file);
                if square == self.en_passant {
                    result += " x ";
                    continue;
                }

                let piece_tuple = self.get_piece_on_square(square);
                if piece_tuple.0 == 0 {
                    result += piece_icons[0][usize::from(piece_tuple.0)];
                } else if piece_tuple.1 == Side::BLACK {
                    result +=
                        piece_icons[Side::BLACK.current()][usize::from(piece_tuple.0)].blue().to_string().as_str();
                } else {
                    result +=
                        piece_icons[Side::WHITE.current()][usize::from(piece_tuple.0)].yellow().to_string().as_str();
                }
            }
            result += format!("| {}", info[(7 - rank) as usize]).as_str();
            result += "\n".to_string().as_str();
        }
        result += " ------------------------\n".to_string().as_str();
        print!("{}\n", result);
    }
}

pub fn create_board(fen: &str) -> Board {
    let mut board = Board::new();
    let splits: Vec<&str> = fen.split_whitespace().collect();

    let ranks = splits[0].split('/');
    for (rank_index, rank) in ranks.enumerate() {
        let mut index = 0;
        let mut file = 0;
        while file < 8 {
            let square = Square::from_coords(7 - rank_index, file);
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
            } else if piece_char == 'n' || piece_char == 'N' {
                board.set_piece_on_square(square, side_to_move, Piece::KNIGHT)
            } else if piece_char == 'b' || piece_char == 'B' {
                board.set_piece_on_square(square, side_to_move, Piece::BISHOP)
            } else if piece_char == 'r' || piece_char == 'R' {
                board.set_piece_on_square(square, side_to_move, Piece::ROOK)
            } else if piece_char == 'q' || piece_char == 'Q' {
                board.set_piece_on_square(square, side_to_move, Piece::QUEEN)
            } else if piece_char == 'k' || piece_char == 'K' {
                board.set_piece_on_square(square, side_to_move, Piece::KING)
            }

            index += 1;
            file += 1;
        }
    }

    if splits[1] == "w" {
        board.side_to_move = Side::WHITE;
    } else {
        board.side_to_move = Side::BLACK;
        board.zobrist.update_side_to_move_hash();
    }

    if board.is_square_attacked(board.get_king_square(board.side_to_move.flipped()), board.side_to_move) {
        print!("Illegal position!\n");
        return Board::new();
    }

    if splits[2].contains('K') {
        board.castle_rights.set_right(CastleRights::WHITE_KING);
        board.zobrist.update_castle_rights_hash(CastleRights::WHITE_KING as usize);
    }
    if splits[2].contains('Q') {
        board.castle_rights.set_right(CastleRights::WHITE_QUEEN);
        board.zobrist.update_castle_rights_hash(CastleRights::WHITE_QUEEN as usize);
    }
    if splits[2].contains('k') {
        board.castle_rights.set_right(CastleRights::BLACK_KING);
        board.zobrist.update_castle_rights_hash(CastleRights::BLACK_KING as usize);
    }
    if splits[2].contains('q') {
        board.castle_rights.set_right(CastleRights::BLACK_QUEEN);
        board.zobrist.update_castle_rights_hash(CastleRights::BLACK_QUEEN as usize);
    }

    board.en_passant = Square::NULL;
    if splits[3] != "-" {
        board.en_passant = Square::from_string(splits[3]);
        board.zobrist.update_en_passant_hash(board.en_passant);
    }

    board.half_moves = 0;

    if splits.len() > 5 {
        board.half_moves = splits[5].parse().unwrap();
    }

    {
        let mut base_rooks = BASE_ROOK_POSITIONS.write().unwrap();
        base_rooks.queen_side = Square::NULL;
        base_rooks.king_side = Square::NULL;

        if false { //for chess960 that are not implemented yet
            let mut rooks = board.get_piece_mask(Piece::ROOK, Side::WHITE);
            if rooks.get_value() > 0 {
                base_rooks.queen_side = rooks.pop_ls1b_square();
            }
            if rooks.get_value() > 0 {
                base_rooks.king_side = rooks.pop_ls1b_square();
            }
        } else {
            base_rooks.queen_side = Square::A1;
            base_rooks.king_side = Square::H1;
        }
    }

    board.checkers = Attacks::generate_checkers_mask(&board);
    board.ortographic_pins = Attacks::generate_ortographic_pins_mask(&board);
    board.diagonal_pins = Attacks::generate_diagonal_pins_mask(&board);

    return board;
}
