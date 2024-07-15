use crate::{
    core::Board,
    neural::{DenseLayer, SparseLayer},
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
#[derive(Clone, Copy, Default)]
pub struct ValueNetwork {
    pub input_layer: SparseLayer<768, 512>,
    pub hidden_layer: DenseLayer<512, 16>,
    pub output_layer: DenseLayer<16, 1>,
}
#[allow(unused)]
impl ValueNetwork {
    #[inline]
    pub fn evaluate(&self, board: &Board) -> f32 {
        let input_layer_result = self.input_layer.forward(&board);
        let hidden_layer_result = self.hidden_layer.forward(&input_layer_result);
        let output_layer_result = self.output_layer.forward(&hidden_layer_result);
        output_layer_result.values()[0]
    }
}
