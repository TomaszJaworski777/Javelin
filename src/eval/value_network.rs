use crate::core::Board;
use crate::neural_core::NetworkLayer;

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
pub struct ValueNetwork {
    input_layer: NetworkLayer<768, 16, SCRELU_FUNCTION>,
    hidden_layer_1: NetworkLayer<16, 2, NO_FUNCTION>,
    output_layer: NetworkLayer<2, 1, NO_FUNCTION>,
}
#[allow(unused)]
impl ValueNetwork {
    pub const fn new() -> Self {
        Self { 
            input_layer: NetworkLayer::new(), 
            hidden_layer_1: NetworkLayer::new(), 
            output_layer: NetworkLayer::new() 
        }
    }

    pub fn set_layer_weights(&mut self, index: usize, weights: Vec<Vec<f32>>) {
        match index {
            0 => self.input_layer.set_weights(weights),
            1 => self.output_layer.set_weights(weights),
            _ => return,
        }
    }

    pub fn set_layer_biases(&mut self, index: usize, biases: Vec<f32>) {
        match index {
            0 => self.input_layer.set_biases(biases),
            1 => self.output_layer.set_biases(biases),
            _ => return,
        }
    }

    pub fn print(&self) {
        self.input_layer.print();
        self.output_layer.print();
    }

    pub fn evaluate(&self, board: &Board) -> f32 {
        let input_layer_result = self.input_layer.feed_input_layer(&board);
        let hidden_layer_output_1 = self.hidden_layer_1.feed_forward(&input_layer_result);
        let output_layer_result = self.output_layer.feed_forward(&hidden_layer_output_1);
        output_layer_result[0]
    }
}
