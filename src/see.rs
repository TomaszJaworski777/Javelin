use crate::core::{Attacks, Board, Move, Piece};

pub struct SEE;
impl SEE {
    pub const QS_MARGIN: i32 = 123;
    pub const PIECE_VALUES: [i32; 6] = [100, 300, 350, 500, 1000, 0];

    pub fn static_exchange_evaluation(board: &Board, mv: Move, threshold: i32) -> bool {
        let from_square = mv.get_from_square();
        let to_square = mv.get_to_square();

        // Next victim is moved piece or promotion type
        let mut next_victim = if mv.is_promotion() {
            mv.get_promotion_piece() + 1
        } else {
            board.get_piece_on_square(from_square).0
        };

        // Balance is the value of the move minus threshold. Function
        // call takes care for Enpass, Promotion and Castling moves.
        let mut balance = estimate_move_value(&board, mv) - threshold;

        // Best case still fails to beat the threshold
        if balance < 0 {
            return false;
        }

        // Worst case is losing the moved piece
        balance -= SEE::PIECE_VALUES[next_victim - 1];

        // If the balance is positive even if losing the moved piece,
        // the exchange is guaranteed to beat the threshold.
        if balance >= 0 {
            return true;
        }

        // Grab sliders for updating revealed attackers
        let bishops = board.get_piece_mask_for_both(Piece::BISHOP) | board.get_piece_mask_for_both(Piece::QUEEN);
        let rooks = board.get_piece_mask_for_both(Piece::ROOK) | board.get_piece_mask_for_both(Piece::QUEEN);

        // Let occupied suppose that the move was actually made
        let mut occupied = board.get_occupancy();
        occupied = (occupied ^ from_square.get_bit()) | to_square.get_bit();
        if mv.is_en_passant() {
            occupied = occupied ^ board.en_passant.get_bit();
        }

        // Get all pieces which attack the target square. And with occupied
        // so that we do not let the same piece attack twice
        let mut attackers = board.all_attackers_to_square(occupied, to_square) & occupied;

        // Now our opponents turn to recapture
        let mut side_to_move = board.side_to_move.flipped();

        loop {

            // If we have no more attackers left we lose
            let my_attackers = attackers & board.get_occupancy_for_side(side_to_move);
            if my_attackers.is_empty() {
                break;
            }

            // Find our weakest piece to attack with
            for new_next_victim in Piece::PAWN..=Piece::KING {
                next_victim = new_next_victim;
                if (my_attackers & board.get_piece_mask_for_both(new_next_victim)).is_not_empty(){
                    break;
                }
            }

            // Remove this attacker from the occupied
            occupied = occupied
                ^ (1u64 << (my_attackers & board.get_piece_mask_for_both(next_victim)).ls1b_square().get_value());

            // A diagonal move may reveal bishop or queen attackers
            if next_victim == Piece::PAWN || next_victim == Piece::BISHOP || next_victim == Piece::QUEEN {
                attackers |= Attacks::get_bishop_attacks_for_square(to_square, occupied) & bishops;
            }

            // A vertical or horizontal move may reveal rook or queen attackers
            if next_victim == Piece::ROOK || next_victim == Piece::QUEEN {
                attackers |= Attacks::get_rook_attacks_for_square(to_square, occupied) & rooks;
            }

            // Make sure we did not add any already used attacks
            attackers &= occupied;

            // Swap the turn
            side_to_move.mut_flip();

            // Negamax the balance and add the value of the next victim
            balance = -balance - 1 - SEE::PIECE_VALUES[next_victim - 1];

            // If the balance is non negative after giving away our piece then we win
            if balance >= 0 {
                // As a slide speed up for move legality checking, if our last attacking
                // piece is a king, and our opponent still has attackers, then we've
                // lost as the move we followed would be illegal
                if next_victim == Piece::KING && (attackers & board.get_occupancy_for_side(side_to_move)).is_not_empty()
                {
                    side_to_move.mut_flip();
                }

                break;
            }
        }

        // Side to move after the loop loses
        board.side_to_move != side_to_move
    }
}

fn estimate_move_value(board: &Board, mv: Move) -> i32 {
    // Start with the value of the piece on the target square
    let target_piece = board.get_piece_on_square(mv.get_to_square()).0;
    let mut value = if target_piece == 0 { 0 } else { SEE::PIECE_VALUES[target_piece - 1] };

    // Factor in the new piece's value and remove our promoted pawn
    if mv.is_promotion() {
        value += SEE::PIECE_VALUES[mv.get_promotion_piece()] - SEE::PIECE_VALUES[0];
    }
    // Target square is encoded as empty for enpass moves
    else if mv.is_en_passant() {
        value = SEE::PIECE_VALUES[0];
    }
    // We encode Castle moves as KxR, so the initial step is wrong
    else if mv.is_king_castle() || mv.is_queen_castle() {
        value = 0;
    }

    return value;
}
