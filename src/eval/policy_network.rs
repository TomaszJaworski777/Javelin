use goober::{activation, layer::SparseConnected, FeedForwardNetwork, Matrix, SparseVector, Vector};

use crate::{
    core::{Board, Move, Side, Bitboard},
    see::SEE
};

#[allow(unused)]
const NO_FUNCTION: u8 = 0;
#[allow(unused)]
const SCRELU_FUNCTION: u8 = 1;
#[allow(unused)]
const RELU_FUNCTION: u8 = 2;
#[allow(unused)]
const SIGMOID_FUNCTION: u8 = 3;

#[repr(C)]
#[derive(Clone, Copy, FeedForwardNetwork)]
pub struct SubNet {
    input_layer: SparseConnected<activation::ReLU, 768, 16>,
}
#[allow(unused)]

impl SubNet {
    pub const fn zeroed() -> Self {
        Self { input_layer: SparseConnected::zeroed() }
    }

    pub fn from_fn<F: FnMut() -> f32>(mut f: F) -> Self {
        let weights = Matrix::from_fn(|_, _| f());
        let biases = Vector::from_fn(|_| f());

        Self { input_layer: SparseConnected::from_raw(weights, biases) }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PolicyNetwork {
    pub subnets: [[SubNet; 2]; 128],
    //pub hce: DenseConnected<activation::Identity, 4, 1>,
}
#[allow(unused)]
impl PolicyNetwork {
    pub const fn zeroed() -> Self {
        Self {
            subnets: [[SubNet::zeroed(); 2]; 128],
            //hce: DenseConnected::zeroed(),
        }
    }

    #[inline]
    pub fn evaluate(&self, board: &Board, mv: &Move, inputs: &SparseVector, threats: Bitboard) -> f32 {
        let flip = if board.side_to_move == Side::WHITE { 0 } else { 56 };

        let threat = usize::from((threats & (1 << mv.get_from_square().get_value())).is_not_empty());
        let from_subnet = &self.subnets[usize::from(mv.get_from_square().get_value() ^ flip)][threat];
        let from_vec = from_subnet.out(inputs);

        let see = usize::from(SEE::static_exchange_evaluation(board, *mv, -108));
        let to_subnet = &self.subnets[64 + usize::from(mv.get_to_square().get_value() ^ flip)][see];
        let to_vec = to_subnet.out(inputs);

        //let hce = self.hce.out(&Self::get_hce_feats(board, mv))[0];

        from_vec.dot(&to_vec) //+ hce
    }

    pub fn get_hce_feats(_: &Board, mov: &Move) -> Vector<4> {
        let mut feats = Vector::zeroed();

        if mov.is_promotion() {
            feats[mov.get_promotion_piece() - 2] = 1.0;
        }

        feats
    }
}
