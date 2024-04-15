#[repr(C)]
#[derive(Clone, Copy)]
pub struct NetworkLayer<const INPUTS: usize, const OUTPUTS: usize, const ACTIVATION: u8> {
    weights: [[f32; INPUTS]; OUTPUTS],
    biases: [f32; OUTPUTS]
}
#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize, const ACTIVATION: u8> NetworkLayer<INPUTS, OUTPUTS, ACTIVATION> {
    pub const fn new() -> Self {
        Self {
            weights: [[0.1; INPUTS]; OUTPUTS],
            biases: [0.1; OUTPUTS]
        }
    }

    pub fn set_weights(&mut self, weights: [[f32; INPUTS]; OUTPUTS]) {
        self.weights = weights
    }

    pub fn set_biases(&mut self, biases: [f32; OUTPUTS]) {
        self.biases = biases;
    }

    pub fn feed_forward(&self, inputs: [f32; INPUTS] )  -> [f32; OUTPUTS] {
        let mut result = [0.0; OUTPUTS];

        for output_index in 0..OUTPUTS {
            for input_index in 0..INPUTS {
                result[output_index] += inputs[input_index] * self.weights[output_index][input_index];
            }
            result[output_index] += self.biases[output_index];

            match ACTIVATION {
                0 => continue,
                1 => result[output_index] = screlu(result[output_index]),
                2 => result[output_index] = relu(result[output_index]),
                3 => result[output_index] = sigmoid(result[output_index]),
                _ => continue,
            }
        }

        result
    }
}

fn screlu(input: f32) -> f32 {
    input.clamp(0.0, 1.0).powi(2)
}

fn relu(input: f32) -> f32 {
    input.max(0.0)
}

fn sigmoid(input: f32) -> f32 {
    1.0 / (1.0 + (-input as f32).exp())
}