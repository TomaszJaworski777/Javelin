use crate::core::{
    attacks::Attacks,
    bitboard::Bitboard,
    board::Board,
    core_structs::{CastleRights, Move, MoveList, Piece, Square},
    rays::Ray,
};

pub struct MoveProvider;
impl MoveProvider {
    pub fn generate_moves<const ONLY_CAPTURE: bool>(move_list: &mut MoveList, board: &Board) {
        if ONLY_CAPTURE {
            generate_king_captures(move_list, &board);
        } else {
            generate_king_moves(move_list, &board);
        }

        let checkers = board.checkers;
        let no_checks = checkers.is_empty();

        if checkers.multiple_one_bits() {
            return;
        } else if !ONLY_CAPTURE && no_checks {
            generate_casting_moves(move_list, &board);
        }

        let move_mask = if no_checks {
            if ONLY_CAPTURE {
                board.get_opponent_occupancy()
            } else {
                !board.get_allied_occupancy()
            }
        } else {
            if ONLY_CAPTURE {
                checkers
            } else {
                let checker = checkers.ls1b_square();
                Ray::get_ray(board.get_king_square(board.side_to_move), checker).include(checker)
            }
        };

        if ONLY_CAPTURE {
            generate_pawn_captures(move_list, board, move_mask);
        } else {
            generate_pawn_moves(move_list, board, move_mask);
        }
        generate_knight_moves(move_list, board, move_mask);
        generate_bishop_moves(move_list, board, move_mask);
        generate_rooks_moves(move_list, board, move_mask);
        //queen moves are already included in bishop and rook attacks
    }
}

fn generate_king_moves(move_list: &mut MoveList, board: &Board) {
    let king_square = board.get_king_square(board.side_to_move);
    let king_moves = Attacks::get_king_attacks_for_square(king_square);
    let occupnacy_mask = board.get_occupancy().exclude(king_square);
    let opponent_occupancy = board.get_opponent_occupancy();
    let king_move_mask = king_moves & !board.get_allied_occupancy();
    //can prune obviously illegal moves prior to this loop (king in check by slider piece can cut potential escape squares)

    for king_move in king_move_mask {
        if !board.is_square_attacked_extended(king_move, board.side_to_move.flipped(), occupnacy_mask) {
            let is_capture = opponent_occupancy.get_bit(king_move);
            let move_mask = if is_capture { Move::CAPTURE_MASK } else { 0 };
            move_list.push(Move::create_move(king_square, king_move, move_mask));
        }
    }
}

fn generate_king_captures(move_list: &mut MoveList, board: &Board) {
    let king_square = board.get_king_square(board.side_to_move);
    let king_moves = Attacks::get_king_attacks_for_square(king_square);
    let occupnacy_mask = board.get_occupancy().exclude(king_square);
    let king_move_mask = king_moves & !board.get_allied_occupancy() & board.get_opponent_occupancy();

    for king_move in king_move_mask {
        if !board.is_square_attacked_extended(king_move, board.side_to_move.flipped(), occupnacy_mask) {
            move_list.push(Move::create_move(king_square, king_move, Move::CAPTURE_MASK));
        }
    }
}

fn generate_casting_moves(move_list: &mut MoveList, board: &Board) {
    let king_position = board.get_king_square(board.side_to_move);
    let side_multiplier = board.side_to_move.current() as u8 * 2;
    let square_offset = board.side_to_move.current() * 56;
    let king_side_rook_position = Square::H1 + square_offset;
    let queen_side_rook_position = Square::A1 + square_offset;
    let occupancy = board.get_occupancy();

    // Helper to check if path is clear and not under attack
    let is_castle_path_clear = |king_destination: Square, rook_position: Square| -> bool {
        let king_ray = Ray::get_ray(king_position, king_destination).include(king_destination);
        let rook_ray = Ray::get_ray(king_position, rook_position);
        let is_line_empty = ((king_ray | rook_ray) & occupancy.exclude(rook_position)).is_empty();
        !board.any_squares_attacked(king_ray, board.side_to_move.flipped()) && is_line_empty
    };

    let king_destination = Square::G1 + square_offset;
    if board.castle_rights.get_right(CastleRights::WHITE_KING + side_multiplier)
        && is_castle_path_clear(king_destination, king_side_rook_position)
    {
        move_list.push(Move::create_move(king_position, king_destination, Move::KING_CASTLE_MASK));
    }

    let king_destination = Square::C1 + square_offset;
    if board.castle_rights.get_right(CastleRights::WHITE_QUEEN + side_multiplier)
        && is_castle_path_clear(king_destination, queen_side_rook_position)
    {
        move_list.push(Move::create_move(king_position, king_destination, Move::QUEEN_CASTLE_MASK));
    }
}

fn generate_pawn_moves(move_list: &mut MoveList, board: &Board, move_mask: Bitboard) {
    let promotion_rank = Bitboard::RANK_7 >> (board.side_to_move.current() * 40) as u32;
    let pawns = board.get_piece_mask(Piece::PAWN, board.side_to_move);

    let movable_pawns = pawns & !board.diagonal_pins;
    let movable_pinned_pawns = movable_pawns & board.ortographic_pins;
    let double_pushable_pawns = movable_pawns & (Bitboard::RANK_2 << (board.side_to_move.current() * 40) as u32);

    let agressive_pawns = pawns & !board.ortographic_pins;
    let agressive_pinned_pawns = agressive_pawns & board.diagonal_pins;

    //normal move, not capture, not pinned, not double push, not promotion
    for not_pinned_pawn in movable_pawns & !double_pushable_pawns & !movable_pinned_pawns & !promotion_rank {
        let pawn_move_mask = Move::PAWN_MOVES[board.side_to_move.current()][not_pinned_pawn.get_value()]
            & !board.get_opponent_occupancy();
        populate_pawn_moves(move_list, not_pinned_pawn, pawn_move_mask & move_mask, 0);
    }

    //normal pinned move, not capture, not double push, not promotion
    for pinned_pawn in movable_pinned_pawns & !double_pushable_pawns & !promotion_rank {
        let pawn_move_mask = Move::PAWN_MOVES[board.side_to_move.current()][pinned_pawn.get_value()]
            & board.ortographic_pins
            & !board.get_opponent_occupancy();
        populate_pawn_moves(move_list, pinned_pawn, pawn_move_mask & move_mask, 0);
    }

    //promotion move, not capture, not pinned
    for promotion_move_pawn in movable_pawns & !movable_pinned_pawns & promotion_rank {
        let pawn_move_mask = Move::PAWN_MOVES[board.side_to_move.current()][promotion_move_pawn.get_value()]
            & !board.get_opponent_occupancy();
        populate_pawn_promotion_moves(move_list, promotion_move_pawn, pawn_move_mask & move_mask, 0);
    }

    //normal move, not capture, not pinned, double push
    for double_push_pawn in double_pushable_pawns & !movable_pinned_pawns {
        if (Square::from_raw(double_push_pawn.get_value() ^ 24).get_bit() & board.get_occupancy()).is_not_empty() {
            continue;
        }
        let pawn_move_mask = Move::PAWN_MOVES[board.side_to_move.current()][double_push_pawn.get_value()]
            & !board.get_opponent_occupancy();
        populate_pawn_moves(move_list, double_push_pawn, pawn_move_mask & move_mask, 0);
        let pawn_move_mask = Move::PAWN_MOVES[board.side_to_move.current()][double_push_pawn.get_value() ^ 24]
            & !board.get_opponent_occupancy();
        populate_pawn_moves(move_list, double_push_pawn, pawn_move_mask & move_mask, Move::DOUBLE_PUSH_MASK);
    }

    //normal pinned move, not capture, double push
    for double_push_pawn in double_pushable_pawns & movable_pinned_pawns {
        if ((double_push_pawn ^ 24).get_bit() & board.get_occupancy()).is_not_empty() {
            continue;
        }
        let pawn_move_mask = Move::PAWN_MOVES[board.side_to_move.current()][double_push_pawn.get_value()]
            & board.ortographic_pins
            & !board.get_opponent_occupancy();
        populate_pawn_moves(move_list, double_push_pawn, pawn_move_mask & move_mask, 0);
        let pawn_move_mask = Move::PAWN_MOVES[board.side_to_move.current()][double_push_pawn.get_value() ^ 24]
            & board.ortographic_pins
            & !board.get_opponent_occupancy();
        populate_pawn_moves(move_list, double_push_pawn, pawn_move_mask & move_mask, Move::DOUBLE_PUSH_MASK);
    }

    //capture, not pinned, not promotion
    for not_pinned_capture in agressive_pawns & !agressive_pinned_pawns & !promotion_rank {
        let pawn_attack_mask = Attacks::get_pawn_attacks_for_square(not_pinned_capture, board.side_to_move)
            & board.get_opponent_occupancy();
        populate_pawn_moves(move_list, not_pinned_capture, pawn_attack_mask & move_mask, Move::CAPTURE_MASK);
    }

    //capture, pinned, not promotion
    for pinned_capture in agressive_pinned_pawns & !promotion_rank {
        let pawn_attack_mask = Attacks::get_pawn_attacks_for_square(pinned_capture, board.side_to_move)
            & board.get_opponent_occupancy()
            & board.diagonal_pins;
        populate_pawn_moves(move_list, pinned_capture, pawn_attack_mask & move_mask, Move::CAPTURE_MASK);
    }

    //capture, not pinned, promotion
    for not_pinned_capture in agressive_pawns & !agressive_pinned_pawns & promotion_rank {
        let pawn_attack_mask = Attacks::get_pawn_attacks_for_square(not_pinned_capture, board.side_to_move)
            & board.get_opponent_occupancy();
        populate_pawn_promotion_moves(move_list, not_pinned_capture, pawn_attack_mask & move_mask, Move::CAPTURE_MASK);
    }

    //capture, pinned, promotion
    for pinned_capture in agressive_pinned_pawns & promotion_rank {
        let pawn_attack_mask = Attacks::get_pawn_attacks_for_square(pinned_capture, board.side_to_move)
            & board.get_opponent_occupancy()
            & board.diagonal_pins;
        populate_pawn_promotion_moves(move_list, pinned_capture, pawn_attack_mask & move_mask, Move::CAPTURE_MASK);
    }

    if board.en_passant == Square::NULL {
        return;
    }

    let process_en_passant_capture_pawn = |target: Square, attacker: Square| -> u64 {
        if (target.get_bit() & move_mask).is_empty() {
            return 0;
        }

        assert_ne!(board.get_king_square(board.side_to_move).get_value(), Square::NULL.get_value());
        let diagonal_xray = Attacks::get_bishop_attacks_for_square(
            board.get_king_square(board.side_to_move),
            board.get_occupancy().exclude(target).exclude(attacker).include(board.en_passant),
        );
        let ortographic_xray = Attacks::get_rook_attacks_for_square(
            board.get_king_square(board.side_to_move),
            board.get_occupancy().exclude(target).exclude(attacker).include(board.en_passant),
        );

        if ((diagonal_xray | ortographic_xray) & target.get_bit()).is_empty() {
            return 1;
        }

        let mut piece_mask = board.get_piece_mask(Piece::BISHOP, board.side_to_move.flipped())
            | board.get_piece_mask(Piece::QUEEN, board.side_to_move.flipped());
        if (diagonal_xray & target.get_bit()).is_not_empty() && (diagonal_xray & piece_mask).is_not_empty() {
            return 0;
        }

        piece_mask = board.get_piece_mask(Piece::ROOK, board.side_to_move.flipped())
            | board.get_piece_mask(Piece::QUEEN, board.side_to_move.flipped());
        if (ortographic_xray & target.get_bit()).is_not_empty() && (ortographic_xray & piece_mask).is_not_empty() {
            return 0;
        }

        1
    };

    //en passant not pinned
    for en_passant_pawn in agressive_pawns & !agressive_pinned_pawns {
        let pawn_attack_mask =
            Attacks::get_pawn_attacks_for_square(en_passant_pawn, board.side_to_move) & board.en_passant.get_bit();
        let attacked_pawn = board.en_passant ^ 8;
        let adjusted_move_mask = Bitboard::from_raw(
            pawn_attack_mask.get_value() * process_en_passant_capture_pawn(attacked_pawn, en_passant_pawn),
        );
        populate_pawn_moves(move_list, en_passant_pawn, adjusted_move_mask, Move::CAPTURE_MASK | Move::EN_PASSANT_MASK);
    }

    //en passant pinned
    for en_passant_pawn in agressive_pinned_pawns {
        let pawn_attack_mask = Attacks::get_pawn_attacks_for_square(en_passant_pawn, board.side_to_move)
            & board.en_passant.get_bit()
            & board.diagonal_pins;
        let attacked_pawn = board.en_passant ^ 8;
        let adjusted_move_mask = Bitboard::from_raw(
            pawn_attack_mask.get_value() * process_en_passant_capture_pawn(attacked_pawn, en_passant_pawn),
        );
        populate_pawn_moves(move_list, en_passant_pawn, adjusted_move_mask, Move::CAPTURE_MASK | Move::EN_PASSANT_MASK);
    }
}

fn generate_pawn_captures(move_list: &mut MoveList, board: &Board, move_mask: Bitboard) {
    let promotion_rank = Bitboard::RANK_7 >> (board.side_to_move.current() * 40) as u32;
    let pawns = board.get_piece_mask(Piece::PAWN, board.side_to_move);
    let agressive_pawns = pawns & !board.ortographic_pins;
    let agressive_pinned_pawns = agressive_pawns & board.diagonal_pins;

    //capture, not pinned, not promotion
    for not_pinned_capture in agressive_pawns & !agressive_pinned_pawns & !promotion_rank {
        let pawn_attack_mask = Attacks::get_pawn_attacks_for_square(not_pinned_capture, board.side_to_move)
            & board.get_opponent_occupancy();
        populate_pawn_moves(move_list, not_pinned_capture, pawn_attack_mask & move_mask, Move::CAPTURE_MASK);
    }

    //capture, pinned, not promotion
    for pinned_capture in agressive_pinned_pawns & !promotion_rank {
        let pawn_attack_mask = Attacks::get_pawn_attacks_for_square(pinned_capture, board.side_to_move)
            & board.get_opponent_occupancy()
            & board.diagonal_pins;
        populate_pawn_moves(move_list, pinned_capture, pawn_attack_mask & move_mask, Move::CAPTURE_MASK);
    }

    //capture, not pinned, promotion
    for not_pinned_capture in agressive_pawns & !agressive_pinned_pawns & promotion_rank {
        let pawn_attack_mask = Attacks::get_pawn_attacks_for_square(not_pinned_capture, board.side_to_move)
            & board.get_opponent_occupancy();
        populate_pawn_promotion_moves(move_list, not_pinned_capture, pawn_attack_mask & move_mask, Move::CAPTURE_MASK);
    }

    //capture, pinned, promotion
    for pinned_capture in agressive_pinned_pawns & promotion_rank {
        let pawn_attack_mask = Attacks::get_pawn_attacks_for_square(pinned_capture, board.side_to_move)
            & board.get_opponent_occupancy()
            & board.diagonal_pins;
        populate_pawn_promotion_moves(move_list, pinned_capture, pawn_attack_mask & move_mask, Move::CAPTURE_MASK);
    }

    if board.en_passant == Square::NULL {
        return;
    }

    let process_en_passant_capture_pawn = |target: Square, attacker: Square| -> u64 {
        if (target.get_bit() & move_mask).is_empty() {
            return 0;
        }

        assert_ne!(board.get_king_square(board.side_to_move).get_value(), Square::NULL.get_value());
        let diagonal_xray = Attacks::get_bishop_attacks_for_square(
            board.get_king_square(board.side_to_move),
            board.get_occupancy().exclude(target).exclude(attacker).include(board.en_passant),
        );
        let ortographic_xray = Attacks::get_rook_attacks_for_square(
            board.get_king_square(board.side_to_move),
            board.get_occupancy().exclude(target).exclude(attacker).include(board.en_passant),
        );

        if ((diagonal_xray | ortographic_xray) & target.get_bit()).is_empty() {
            return 1;
        }

        let mut piece_mask = board.get_piece_mask(Piece::BISHOP, board.side_to_move.flipped())
            | board.get_piece_mask(Piece::QUEEN, board.side_to_move.flipped());
        if (diagonal_xray & target.get_bit()).is_not_empty() && (diagonal_xray & piece_mask).is_not_empty() {
            return 0;
        }

        piece_mask = board.get_piece_mask(Piece::ROOK, board.side_to_move.flipped())
            | board.get_piece_mask(Piece::QUEEN, board.side_to_move.flipped());
        if (ortographic_xray & target.get_bit()).is_not_empty() && (ortographic_xray & piece_mask).is_not_empty() {
            return 0;
        }

        1
    };

    //en passant not pinned
    for en_passant_pawn in agressive_pawns & !agressive_pinned_pawns {
        let pawn_attack_mask =
            Attacks::get_pawn_attacks_for_square(en_passant_pawn, board.side_to_move) & board.en_passant.get_bit();
        let attacked_pawn = board.en_passant ^ 8;
        let adjusted_move_mask = Bitboard::from_raw(
            pawn_attack_mask.get_value() * process_en_passant_capture_pawn(attacked_pawn, en_passant_pawn),
        );
        populate_pawn_moves(move_list, en_passant_pawn, adjusted_move_mask, Move::CAPTURE_MASK | Move::EN_PASSANT_MASK);
    }

    //en passant pinned
    for en_passant_pawn in agressive_pinned_pawns {
        let pawn_attack_mask = Attacks::get_pawn_attacks_for_square(en_passant_pawn, board.side_to_move)
            & board.en_passant.get_bit()
            & board.diagonal_pins;
        let attacked_pawn = board.en_passant ^ 8;
        let adjusted_move_mask = Bitboard::from_raw(
            pawn_attack_mask.get_value() * process_en_passant_capture_pawn(attacked_pawn, en_passant_pawn),
        );
        populate_pawn_moves(move_list, en_passant_pawn, adjusted_move_mask, Move::CAPTURE_MASK | Move::EN_PASSANT_MASK);
    }
}

fn generate_knight_moves(move_list: &mut MoveList, board: &Board, move_mask: Bitboard) {
    let pin_mask = board.ortographic_pins | board.diagonal_pins;
    for knight_square in board.get_piece_mask(Piece::KNIGHT, board.side_to_move) & !pin_mask {
        let knight_attacks = Attacks::get_knight_attacks_for_square(knight_square) & move_mask;
        populate_piece_moves(move_list, board, knight_square, knight_attacks);
    }
}

fn generate_bishop_moves(move_list: &mut MoveList, board: &Board, move_mask: Bitboard) {
    let piece_mask = board.get_piece_mask(Piece::BISHOP, board.side_to_move)
        | board.get_piece_mask(Piece::QUEEN, board.side_to_move);
    let movable_bishops = piece_mask & !board.ortographic_pins;
    let pinned_bishops = movable_bishops & board.diagonal_pins;
    for bishop_square in movable_bishops & !pinned_bishops {
        let bishop_attacks = Attacks::get_bishop_attacks_for_square(bishop_square, board.get_occupancy()) & move_mask;
        populate_piece_moves(move_list, board, bishop_square, bishop_attacks);
    }
    for bishop_square in pinned_bishops {
        let bishop_attacks = Attacks::get_bishop_attacks_for_square(bishop_square, board.get_occupancy())
            & move_mask
            & board.diagonal_pins;
        populate_piece_moves(move_list, board, bishop_square, bishop_attacks);
    }
}

fn generate_rooks_moves(move_list: &mut MoveList, board: &Board, move_mask: Bitboard) {
    let piece_mask =
        board.get_piece_mask(Piece::ROOK, board.side_to_move) | board.get_piece_mask(Piece::QUEEN, board.side_to_move);
    let movable_rooks = piece_mask & !board.diagonal_pins;
    let pinned_rooks = movable_rooks & board.ortographic_pins;
    for rook_square in movable_rooks & !pinned_rooks {
        let bishop_attacks = Attacks::get_rook_attacks_for_square(rook_square, board.get_occupancy()) & move_mask;
        populate_piece_moves(move_list, board, rook_square, bishop_attacks);
    }
    for rook_square in pinned_rooks {
        let bishop_attacks = Attacks::get_rook_attacks_for_square(rook_square, board.get_occupancy())
            & move_mask
            & board.ortographic_pins;
        populate_piece_moves(move_list, board, rook_square, bishop_attacks);
    }
}

fn populate_pawn_promotion_moves(move_list: &mut MoveList, piece_position: Square, moves: Bitboard, move_masks: u16) {
    for pawn_move in moves {
        move_list.push(Move::create_move(piece_position, pawn_move, Move::PROMOTION_KNIGHT_MASK | move_masks));
        move_list.push(Move::create_move(piece_position, pawn_move, Move::PROMOTION_BISHOP_MASK | move_masks));
        move_list.push(Move::create_move(piece_position, pawn_move, Move::PROMOTION_ROOK_MASK | move_masks));
        move_list.push(Move::create_move(piece_position, pawn_move, Move::PROMOTION_QUEEN_MASK | move_masks));
    }
}

fn populate_pawn_moves(move_list: &mut MoveList, piece_position: Square, moves: Bitboard, move_masks: u16) {
    for pawn_move in moves {
        move_list.push(Move::create_move(piece_position, pawn_move, move_masks));
    }
}

fn populate_piece_moves(move_list: &mut MoveList, board: &Board, piece_position: Square, moves: Bitboard) {
    let quiet_moves = moves & !board.get_occupancy();
    let captures = moves & board.get_opponent_occupancy();

    for piece_move in quiet_moves {
        move_list.push(Move::create_move(piece_position, piece_move, 0));
    }

    for piece_move in captures {
        move_list.push(Move::create_move(piece_position, piece_move, Move::CAPTURE_MASK));
    }
}
