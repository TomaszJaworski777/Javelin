mod pesto;
mod policy_network;
mod value_network;

use goober::SparseVector;

#[allow(unused)]
pub use policy_network::PolicyNetwork;
#[allow(unused)]
pub use policy_network::SubNet;
use spear::Bitboard;
use spear::ChessBoard;
use spear::Move;
use spear::Piece;
use spear::Side;
#[allow(unused)]
pub use value_network::ValueNetwork;

pub const VALUE_NETWORK: ValueNetwork =
    unsafe { std::mem::transmute(*include_bytes!("../resources/nets/value_011.net")) };

pub const POLICY_NETWORK: PolicyNetwork =
    unsafe { std::mem::transmute(*include_bytes!("../resources/nets/policy_008.net")) };

pub struct Evaluation;
impl Evaluation {
    #[inline]
    pub fn evaluate<const STM_WHITE: bool, const NSTM_WHITE: bool>(board: &ChessBoard) -> i32 {
        (VALUE_NETWORK.evaluate::<STM_WHITE, NSTM_WHITE>(&board) * 400.0) as i32
    }

    #[inline]
    pub fn get_policy_value(board: &ChessBoard, mv: Move, inputs: &SparseVector, threats: Bitboard) -> f32 {
        POLICY_NETWORK.evaluate(&board, mv, &inputs, threats)
    }

    pub fn get_policy_inputs<const STM_WHITE: bool, const NSTM_WHITE: bool>(board: &ChessBoard) -> SparseVector {
        let mut result = SparseVector::with_capacity(32);
        let flip = board.side_to_move() == Side::BLACK;

        for piece in Piece::PAWN.get_raw()..=Piece::KING.get_raw() {
            let piece = Piece::from_raw(piece);
            let piece_index = 64 * piece.get_raw() as usize;

            let mut stm_bitboard = board.get_piece_mask_for_side::<STM_WHITE>(piece);
            let mut nstm_bitboard = board.get_piece_mask_for_side::<NSTM_WHITE>(piece);

            if flip {
                stm_bitboard = stm_bitboard.flip();
                nstm_bitboard = nstm_bitboard.flip();
            }

            stm_bitboard.map(|square| result.push(piece_index + square.get_raw() as usize));

            nstm_bitboard.map(|square| result.push(384 + piece_index + square.get_raw() as usize));
        }

        result
    }
}
