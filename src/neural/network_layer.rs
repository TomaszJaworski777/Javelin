use std::marker::PhantomData;

use crate::core::{Board, Piece, Side};

use super::activation::ActivationFunction;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DenseLayer<const INPUTS: usize, const OUTPUTS: usize, Activation>
where
    Activation: ActivationFunction,
{
    pub layer: NetworkLayer<INPUTS, OUTPUTS>,
    _marker: PhantomData<Activation>,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SparseLayer<const INPUTS: usize, const OUTPUTS: usize, Activation>
where
    Activation: ActivationFunction,
{
    pub layer: NetworkLayer<INPUTS, OUTPUTS>,
    _marker: PhantomData<Activation>,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CustomLayer<const INPUTS: usize, const OUTPUTS: usize, Activation>
where
    Activation: ActivationFunction,
{
    pub layer: NetworkLayer<INPUTS, OUTPUTS>,
    _marker: PhantomData<Activation>,
}

#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize, Activation> DenseLayer<INPUTS, OUTPUTS, Activation>
where
    Activation: ActivationFunction,
{
    pub fn layer(&self) -> &NetworkLayer<INPUTS, OUTPUTS> {
        &self.layer
    }

    pub fn layer_mut(&mut self) -> &mut NetworkLayer<INPUTS, OUTPUTS> {
        &mut self.layer
    }

    pub fn forward(&self, inputs: &[f32; INPUTS]) -> [f32; OUTPUTS] {
        let mut result = self.layer.biases;

        for input_index in 0..INPUTS {
            for output_index in 0..OUTPUTS {
                let input = Activation::execute(inputs[input_index]);
                result[output_index] += input * self.layer.weights[input_index][output_index];
            }
        }

        result
    }
}

#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize, Activation> SparseLayer<INPUTS, OUTPUTS, Activation>
where
    Activation: ActivationFunction,
{
    pub const fn layer(&self) -> &NetworkLayer<INPUTS, OUTPUTS> {
        &self.layer
    }

    pub fn layer_mut(&mut self) -> &mut NetworkLayer<INPUTS, OUTPUTS> {
        &mut self.layer
    }

    pub fn forward(&self, board: &Board) -> [f32; OUTPUTS] {
        let mut result = self.layer.biases;

        Self::map_value_inputs(board, |weight_index| {
            for (i, weight) in result.iter_mut().zip(&self.layer.weights[weight_index]) {
                *i += *weight;
            }
        });

        result
    }

    fn map_value_inputs<F: FnMut(usize)>(board: &Board, mut method: F) {
        let flip = board.side_to_move == Side::BLACK;

        for piece in Piece::PAWN..=Piece::KING {
            let piece_index = 64 * (piece - Piece::PAWN);

            let mut stm_bitboard = board.get_piece_mask(piece, board.side_to_move);
            let mut nstm_bitboard = board.get_piece_mask(piece, board.side_to_move.flipped());

            if flip {
                stm_bitboard = stm_bitboard.flip();
                nstm_bitboard = nstm_bitboard.flip();
            }

            for square in stm_bitboard {
                method(piece_index + square.get_value())
            }

            for square in nstm_bitboard {
                method(384 + piece_index + square.get_value())
            }
        }
    }
}

#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize, Activation> CustomLayer<INPUTS, OUTPUTS, Activation>
where
    Activation: ActivationFunction,
{
    pub fn layer(&self) -> &NetworkLayer<INPUTS, OUTPUTS> {
        &self.layer
    }

    pub fn layer_mut(&mut self) -> &mut NetworkLayer<INPUTS, OUTPUTS> {
        &mut self.layer
    }

    pub fn forward(
        &self,
        method: fn(
            weights: [[f32; OUTPUTS]; INPUTS],
            biases: [f32; OUTPUTS],
            activation: fn(f32) -> f32,
        ) -> [f32; OUTPUTS],
    ) -> [f32; OUTPUTS] {
        method(self.layer.weights, self.layer.biases, Activation::execute)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NetworkLayer<const INPUTS: usize, const OUTPUTS: usize> {
    pub weights: [[f32; OUTPUTS]; INPUTS],
    biases: [f32; OUTPUTS],
}

impl<const INPUTS: usize, const OUTPUTS: usize> Default for NetworkLayer<INPUTS, OUTPUTS> {
    fn default() -> Self {
        Self { weights: [[0.0; OUTPUTS]; INPUTS], biases: [0.0; OUTPUTS] }
    }
}

#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize> NetworkLayer<INPUTS, OUTPUTS> {
    pub fn set_weights(&mut self, weights: Vec<Vec<f32>>) {
        for input_index in 0..INPUTS {
            for output_index in 0..OUTPUTS {
                self.weights[input_index][output_index] = weights[input_index][output_index];
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
}
