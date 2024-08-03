use spear::{ChessBoard, Piece, Side};

use super::{activation::ActivationFunction, ScReLUActivation};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DenseLayer<const INPUTS: usize, const OUTPUTS: usize> {
    pub layer: NetworkLayer<INPUTS, OUTPUTS>,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SparseLayer<const INPUTS: usize, const OUTPUTS: usize> {
    pub layer: NetworkLayer<INPUTS, OUTPUTS>,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CustomLayer<const INPUTS: usize, const OUTPUTS: usize> {
    pub layer: NetworkLayer<INPUTS, OUTPUTS>,
}

#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize> DenseLayer<INPUTS, OUTPUTS> {
    pub fn layer(&self) -> &NetworkLayer<INPUTS, OUTPUTS> {
        &self.layer
    }

    pub fn layer_mut(&mut self) -> &mut NetworkLayer<INPUTS, OUTPUTS> {
        &mut self.layer
    }

    pub fn forward(&self, inputs: &Accumulator<INPUTS>) -> Accumulator<OUTPUTS> {
        let mut result = self.layer.biases;

        for (neuron, weights) in inputs.vals.iter().zip(self.layer.weights.iter()) {
            let activated = ScReLUActivation::execute(*neuron);
            result.madd(activated, weights);
        }

        result
    }
}

#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize> SparseLayer<INPUTS, OUTPUTS> {
    pub const fn layer(&self) -> &NetworkLayer<INPUTS, OUTPUTS> {
        &self.layer
    }

    pub fn layer_mut(&mut self) -> &mut NetworkLayer<INPUTS, OUTPUTS> {
        &mut self.layer
    }

    pub fn forward<const STM_WHITE: bool, const NSTM_WHITE: bool>(&self, board: &ChessBoard) -> Accumulator<OUTPUTS> {
        let mut result = self.layer.biases;

        Self::map_value_inputs::<_, STM_WHITE, NSTM_WHITE>(board, |weight_index| {
            for (i, weight) in result.vals.iter_mut().zip(&self.layer.weights[weight_index].vals) {
                *i += *weight;
            }
        });

        result
    }

    fn map_value_inputs<F: FnMut(usize), const STM_WHITE: bool, const NSTM_WHITE: bool>(
        board: &ChessBoard,
        mut method: F,
    ) {
        let flip = board.side_to_move() == Side::BLACK;
        let horizontal_mirror = if board.get_king_square::<STM_WHITE>().get_raw() % 8 > 3 { 7 } else { 0 };

        for piece in Piece::PAWN.get_raw()..=Piece::KING.get_raw() {
            let piece = Piece::from_raw(piece);
            let piece_index = 64 * piece.get_raw() as usize;

            let mut stm_bitboard = board.get_piece_mask_for_side::<STM_WHITE>(piece);
            let mut nstm_bitboard = board.get_piece_mask_for_side::<NSTM_WHITE>(piece);

            if flip {
                stm_bitboard = stm_bitboard.flip();
                nstm_bitboard = nstm_bitboard.flip();
            }

            stm_bitboard.map(|square| method(piece_index + (square.get_raw() as usize ^ horizontal_mirror)));

            nstm_bitboard
                .map(|square| method(384 + piece_index + (square.get_raw() as usize ^ horizontal_mirror)));
        }
    }
}

#[allow(unused)]
impl<const INPUTS: usize, const OUTPUTS: usize> CustomLayer<INPUTS, OUTPUTS> {
    pub fn layer(&self) -> &NetworkLayer<INPUTS, OUTPUTS> {
        &self.layer
    }

    pub fn layer_mut(&mut self) -> &mut NetworkLayer<INPUTS, OUTPUTS> {
        &mut self.layer
    }

    pub fn forward(
        &self,
        method: fn(
            weights: [Accumulator<OUTPUTS>; INPUTS],
            biases: Accumulator<OUTPUTS>,
            activation: fn(f32) -> f32,
        ) -> [f32; OUTPUTS],
    ) -> [f32; OUTPUTS] {
        method(self.layer.weights, self.layer.biases, ScReLUActivation::execute)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NetworkLayer<const INPUTS: usize, const OUTPUTS: usize> {
    weights: [Accumulator<OUTPUTS>; INPUTS],
    biases: Accumulator<OUTPUTS>,
}

impl<const INPUTS: usize, const OUTPUTS: usize> Default for NetworkLayer<INPUTS, OUTPUTS> {
    fn default() -> Self {
        Self { weights: [Accumulator::default(); INPUTS], biases: Accumulator::default() }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Accumulator<const HIDDEN: usize> {
    vals: [f32; HIDDEN],
}

impl<const SIZE: usize> Default for Accumulator<SIZE> {
    fn default() -> Self {
        Self { vals: [0.0; SIZE] }
    }
}

impl<const HIDDEN: usize> Accumulator<HIDDEN> {
    fn madd(&mut self, mul: f32, other: &Self) {
        for (i, &j) in self.vals.iter_mut().zip(other.vals.iter()) {
            *i += mul * j;
        }
    }

    pub fn values(&self) -> [f32; HIDDEN] {
        self.vals
    }
}
