use crate::core::{Board, Side};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NetworkLayer<const INPUTS: usize, const OUTPUTS: usize, const ACTIVATION: u8> {
    weights: [[f32; INPUTS]; OUTPUTS],
    biases: [f32; OUTPUTS],
}
#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize, const ACTIVATION: u8> NetworkLayer<INPUTS, OUTPUTS, ACTIVATION> {
    pub const fn new() -> Self {
        Self { weights: [[0.1; INPUTS]; OUTPUTS], biases: [0.1; OUTPUTS] }
    }

    pub fn set_weights(&mut self, weights: Vec<Vec<f32>>) {
        for output_index in 0..OUTPUTS {
            for input_index in 0..INPUTS {
                self.weights[output_index][input_index] = weights[output_index][input_index];
            }
        }
    }

    pub fn set_biases(&mut self, biases: Vec<f32>) {
        for output_index in 0..OUTPUTS {
            self.biases[output_index] = biases[output_index];
        }
    }

    pub fn print(&self) {
        println!("\nWeights:");
        for weight_index in 0..self.weights.len() {
            for (index, weight) in self.weights[weight_index].iter().enumerate() {
                if index != 0 && index % 8 == 0 {
                    print!("\n");
                }
                print!("{}, ", weight);
            }
        }
        println!("\nBiases:");
        for bias_index in 0..self.biases.len() {
            if bias_index != 0 && bias_index % 8 == 0 {
                print!("\n");
            }
            print!("{}, ", self.biases[bias_index]);
        }
    }

    pub fn feed_forward(&self, inputs: &[f32; INPUTS]) -> [f32; OUTPUTS] {
        let mut result = self.biases;

        for output_index in 0..OUTPUTS {
            for input_index in 0..INPUTS {
                let input = match ACTIVATION {
                    0 => inputs[input_index],
                    1 => screlu(inputs[input_index]),
                    2 => relu(inputs[input_index]),
                    3 => sigmoid(inputs[input_index]),
                    _ => inputs[input_index],
                };
                
                result[output_index] += input * self.weights[output_index][input_index];
            }
        }

        result
    }

    pub fn feed_input_layer(&self, board: &Board) -> [f32; OUTPUTS] {
        let mut result = self.biases;

        for piece_index in 1..=6 {
            let mut stm_bitboard = board.get_piece_mask(piece_index, board.side_to_move);
            let mut nstm_bitboard = board.get_piece_mask(piece_index, board.side_to_move.flipped());

            if board.side_to_move == Side::BLACK {
                stm_bitboard = stm_bitboard.flip();
                nstm_bitboard = nstm_bitboard.flip();
            }

            for output_index in 0..OUTPUTS {
                for square in stm_bitboard {
                    result[output_index] += self.weights[output_index][(piece_index - 1) * 64 + square.get_value()];
                }

                for square in nstm_bitboard {
                    result[output_index] +=
                        self.weights[output_index][384 + (piece_index - 1) * 64 + square.get_value()];
                }
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
