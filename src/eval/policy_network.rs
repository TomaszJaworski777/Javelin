use crate::neural_core::NetworkLayer;
use crate::core::Board;

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
    input_layer: NetworkLayer<768, 384, NO_FUNCTION>,
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

    pub fn evaluate(&self, board: &Board, mask: &[bool; 384]) -> Vec<f32> {
        let input_layer_result = self.input_layer.feed_input_layer(&board);
        let masked_output: Vec<f32> = input_layer_result.iter()
        .zip(mask.iter())
        .map(|(&x, &y)| if y { x } else { f32::NEG_INFINITY })
        .collect();
        softmax(&masked_output)
    }
}

fn softmax(x: &Vec<f32>) -> Vec<f32> {
    if x.is_empty() {
        return Vec::new();
    }
    let max_val = x.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let exps: Vec<f32> = x.iter().map(|&num| if num == f32::NEG_INFINITY { 0.0 } else { (num - max_val).exp() }).collect();
    let sum_exps: f32 = exps.iter().sum();
    exps.iter().map(|&exp| exp / sum_exps).collect()
}
