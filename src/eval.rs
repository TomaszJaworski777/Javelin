mod pesto;
mod policy_network;
mod value_network;

use crate::core::{Board, MoveList, Side};

#[allow(unused)]
pub use policy_network::PolicyNetwork;
#[allow(unused)]
pub use value_network::ValueNetwork;

pub const VALUE_NETWORK: ValueNetwork =
    unsafe { std::mem::transmute(*include_bytes!("../resources/nets/value_001b.net")) };

pub const POLICY_NETWORK: PolicyNetwork =
    unsafe { std::mem::transmute(*include_bytes!("../resources/nets/policy_001.net")) };

pub struct Evaluation;
impl Evaluation {
    pub fn evaluate(board: &Board) -> i32 {
        (VALUE_NETWORK.evaluate(&board) * 400.0) as i32
    }

    pub fn get_policy_values(board: &Board, move_list: &MoveList) -> Vec<f32> {
        let mut mask = [false; 384];
        for mv in move_list {
            let base_index = (board.get_piece_on_square(mv.get_from_square()).0 - 1) * 64;
            let index = base_index
                + if board.side_to_move == Side::WHITE {
                    mv.get_to_square().get_value()
                } else {
                    mv.get_to_square().get_value() ^ 56
                };
            mask[index] = true;
        }
        POLICY_NETWORK.evaluate(&board, &mask)
    }
}
