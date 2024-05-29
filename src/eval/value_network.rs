use crate::{
    core::Board,
    neural::{DenseLayer, NoActivation, ScReLUActivation, SpareLayer},
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
    input_layer: SpareLayer<768, 16, NoActivation>,
    output_layer: DenseLayer<16, 1, ScReLUActivation>,
}
#[allow(unused)]
impl ValueNetwork {
    pub fn set_layer_weights(&mut self, index: usize, weights: Vec<Vec<f32>>) {
        match index {
            0 => self.input_layer.layer_mut().set_weights(weights),
            1 => self.output_layer.layer_mut().set_weights(weights),
            _ => return,
        }
    }

    pub fn set_layer_biases(&mut self, index: usize, biases: Vec<f32>) {
        match index {
            0 => self.input_layer.layer_mut().set_biases(biases),
            1 => self.output_layer.layer_mut().set_biases(biases),
            _ => return,
        }
    }

    pub fn print(&self) {
        self.input_layer.layer().print();
        self.output_layer.layer().print();
    }

    pub fn evaluate(&self, board: &Board) -> f32 {
        let input_layer_result = self.input_layer.forward(&board);
        let output_layer_result = self.output_layer.forward(&input_layer_result);
        output_layer_result[0]
    }
}
