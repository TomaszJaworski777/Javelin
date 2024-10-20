mod pesto;
mod policy_network;
mod value_network;

use crate::core::{Board, Move, Piece, Side, Bitboard};

use goober::SparseVector;

#[allow(unused)]
pub use policy_network::PolicyNetwork;
#[allow(unused)]
pub use policy_network::SubNet;
#[allow(unused)]
pub use value_network::ValueNetwork;

pub const VALUE_NETWORK: ValueNetwork =
    unsafe { std::mem::transmute(*include_bytes!("../resources/nets/value_011.net")) };

pub const POLICY_NETWORK: PolicyNetwork =
    unsafe { std::mem::transmute(*include_bytes!("../resources/nets/policy_008.net")) };

pub struct Evaluation;
impl Evaluation {
    #[inline]
    pub fn evaluate(board: &Board) -> i32 {
        (VALUE_NETWORK.evaluate(&board) * 400.0) as i32
    }

    #[inline]
    pub fn get_policy_value(board: &Board, mv: &Move, inputs: &SparseVector, threats: Bitboard) -> f32 {
        POLICY_NETWORK.evaluate(&board, &mv, &inputs, threats)
    }

    pub fn get_policy_inputs(board: &Board) -> SparseVector {
        let mut result = SparseVector::with_capacity(32);
        let flip = board.side_to_move == Side::BLACK;

        for piece in Piece::PAWN..=Piece::KING {
            let piece_index = 64 * (piece - Piece::PAWN);

            let mut stm_bitboard = board.get_piece_mask(piece, board.side_to_move);
            let mut nstm_bitboard = board.get_piece_mask(piece, board.side_to_move.flipped());

            if flip {
                stm_bitboard = stm_bitboard.flip();
                nstm_bitboard = nstm_bitboard.flip();
            }

            for square in stm_bitboard {
                result.push(piece_index + square.get_value());
            }

            for square in nstm_bitboard {
                result.push(384 + piece_index + square.get_value());
            }
        }

        result
    }
}
