use crate::neural_core::NetworkLayer;
use crate::core::{Board, Move};

#[allow(unused)]
const NO_FUNCTION: u8 = 0;
#[allow(unused)]
const SCRELU_FUNCTION: u8 = 1;
#[allow(unused)]
const RELU_FUNCTION: u8 = 2;
#[allow(unused)]
const SIGMOID_FUNCTION: u8 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PolicyNetwork {
    input_layer: NetworkLayer<768, 384, SIGMOID_FUNCTION>,
}
#[allow(unused)]
impl PolicyNetwork {
    pub const fn new() -> Self {
        Self { input_layer: NetworkLayer::new() }
    }

    pub fn set_layer_weights(&mut self, index: usize, weights: Vec<Vec<f32>>) {
        match index {
            0 => self.input_layer.set_weights(weights),
            _ => return,
        }
    }

    pub fn set_layer_biases(&mut self, index: usize, biases: Vec<f32>) {
        match index {
            0 => self.input_layer.set_biases(biases),
            _ => return,
        }
    }

    pub fn evaluate(&self, board: &Board, mv: Move) -> f32 {
        let input_layer_result = self.input_layer.feed_input_layer(&board);
        input_layer_result[64 * (board.get_piece_on_square(mv.get_from_square()).0 - 1) + mv.get_to_square().get_value()]
    }
}
