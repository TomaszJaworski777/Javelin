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

    pub fn feed_forward(&self, inputs: &[f32; INPUTS]) -> [f32; OUTPUTS] {
        let mut result = [0.0; OUTPUTS];

        for output_index in 0..OUTPUTS {
            for input_index in 0..INPUTS {
                result[output_index] += inputs[input_index] * self.weights[output_index][input_index];
            }
            result[output_index] += self.biases[output_index];

            result[output_index] = match ACTIVATION {
                0 => continue,
                1 => screlu(result[output_index]),
                2 => relu(result[output_index]),
                3 => sigmoid(result[output_index]),
                _ => continue,
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
                    result[output_index] += self.weights[output_index][384 + (piece_index - 1) * 64 + square.get_value()];
                }

                result[output_index] = match ACTIVATION {
                    0 => continue,
                    1 => screlu(result[output_index]),
                    2 => relu(result[output_index]),
                    3 => sigmoid(result[output_index]),
                    _ => continue,
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
