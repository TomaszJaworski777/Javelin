use std::marker::PhantomData;

use crate::core::{Board, Side};

use super::activation::ActivationFunction;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DenseLayer<const INPUTS: usize, const OUTPUTS: usize, Activation>
where
    Activation: ActivationFunction,
{
    layer: NetworkLayer<INPUTS, OUTPUTS>,
    _marker: PhantomData<Activation>,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SparseLayer<const INPUTS: usize, const OUTPUTS: usize, Activation>
where
    Activation: ActivationFunction,
{
    layer: NetworkLayer<INPUTS, OUTPUTS>,
    _marker: PhantomData<Activation>,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CustomLayer<const INPUTS: usize, const OUTPUTS: usize, Activation>
where
    Activation: ActivationFunction,
{
    layer: NetworkLayer<INPUTS, OUTPUTS>,
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
                result[output_index] += input * self.layer.weights[output_index][input_index];
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
    pub fn layer(&self) -> &NetworkLayer<INPUTS, OUTPUTS> {
        &self.layer
    }

    pub fn layer_mut(&mut self) -> &mut NetworkLayer<INPUTS, OUTPUTS> {
        &mut self.layer
    }

    pub fn forward(&self, board: &Board) -> [f32; OUTPUTS] {
        let mut result = self.layer.biases;

        for piece_index in 1..=6 {
            let mut stm_bitboard = board.get_piece_mask(piece_index, board.side_to_move);
            let mut nstm_bitboard = board.get_piece_mask(piece_index, board.side_to_move.flipped());

            if board.side_to_move == Side::BLACK {
                stm_bitboard = stm_bitboard.flip();
                nstm_bitboard = nstm_bitboard.flip();
            }

            for output_index in 0..OUTPUTS {
                for square in stm_bitboard {
                    result[output_index] +=
                        self.layer.weights[output_index][(piece_index - 1) * 64 + square.get_value()];
                }

                for square in nstm_bitboard {
                    result[output_index] +=
                        self.layer.weights[output_index][384 + (piece_index - 1) * 64 + square.get_value()];
                }
            }
        }

        result
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
            weights: [[f32; INPUTS]; OUTPUTS],
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
    weights: [[f32; INPUTS]; OUTPUTS],
    biases: [f32; OUTPUTS],
}

impl<const INPUTS: usize, const OUTPUTS: usize> Default for NetworkLayer<INPUTS, OUTPUTS> {
    fn default() -> Self {
        Self { weights: [[0.1; INPUTS]; OUTPUTS], biases: [0.1; OUTPUTS] }
    }
}

#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize> NetworkLayer<INPUTS, OUTPUTS> {
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
}
