use crate::core::{Board, Piece, Side};

use super::{activation::ActivationFunction, ScReLUActivation};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DenseLayer<const INPUTS: usize, const OUTPUTS: usize>
{
    pub layer: NetworkLayer<INPUTS, OUTPUTS>
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SparseLayer<const INPUTS: usize, const OUTPUTS: usize>
{
    pub layer: NetworkLayer<INPUTS, OUTPUTS>
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CustomLayer<const INPUTS: usize, const OUTPUTS: usize>
{
    pub layer: NetworkLayer<INPUTS, OUTPUTS>
}

#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize> DenseLayer<INPUTS, OUTPUTS>
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
                let input = ScReLUActivation::execute(inputs[input_index]);
                result[output_index] += input * self.layer.weights[input_index][output_index];
            }
        }

        result
    }
}

#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize> SparseLayer<INPUTS, OUTPUTS>
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
        let horizontal_mirror = if board.get_king_square(board.side_to_move).get_value() % 8 > 3 { 7 } else { 0 };

        for piece in Piece::PAWN..=Piece::KING {
            let piece_index = 64 * (piece - Piece::PAWN);

            let mut stm_bitboard = board.get_piece_mask(piece, board.side_to_move);
            let mut nstm_bitboard = board.get_piece_mask(piece, board.side_to_move.flipped());

            if flip {
                stm_bitboard = stm_bitboard.flip();
                nstm_bitboard = nstm_bitboard.flip();
            }

            for square in stm_bitboard {
                method(piece_index + (square.get_value() ^ horizontal_mirror))
            }

            for square in nstm_bitboard {
                method(384 + piece_index + (square.get_value() ^ horizontal_mirror))
            }
        }
    }
}

#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize> CustomLayer<INPUTS, OUTPUTS>
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
        method(self.layer.weights, self.layer.biases, ScReLUActivation::execute)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NetworkLayer<const INPUTS: usize, const OUTPUTS: usize> {
    weights: [[f32; OUTPUTS]; INPUTS],
    biases: [f32; OUTPUTS],
}

impl<const INPUTS: usize, const OUTPUTS: usize> Default for NetworkLayer<INPUTS, OUTPUTS> {
    fn default() -> Self {
        Self { weights: [[0.0; OUTPUTS]; INPUTS], biases: [0.0; OUTPUTS] }
    }
}
