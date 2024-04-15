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
    input_layer: NetworkLayer<768, 64, SCRELU_FUNCTION>,
    output_layer: NetworkLayer<64, 1, SIGMOID_FUNCTION>
}
#[allow(unused)]
impl ValueNetwork {
    pub const fn new() -> Self {
        Self {
            input_layer: NetworkLayer::new(),
            output_layer: NetworkLayer::new(),
        }
    }

    pub const fn load(path: &str) -> Self {
        Self {
            input_layer: NetworkLayer::new(),
            output_layer: NetworkLayer::new(),
        }
    }

    pub fn access_input_layer(&mut self) -> &mut NetworkLayer<768, 64, SCRELU_FUNCTION>{
        &mut self.input_layer
    }

    pub fn access_output_layer(&mut self) -> &mut NetworkLayer<64, 1, SIGMOID_FUNCTION>{
        &mut self.output_layer
    }

    pub fn evaluate(&self, inputs: [f32; 768] )  -> f32 {
        let input_layer_result = self.input_layer.feed_forward(inputs);
        let output_layer_result = self.output_layer.feed_forward(input_layer_result);
        output_layer_result[0]
    }
}