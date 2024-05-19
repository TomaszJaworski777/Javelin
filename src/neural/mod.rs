mod activation;
mod network_layer;

#[allow(unused)]
pub use activation::{NoActivation, ScReLUActivation, ReLUActivation, SigmoidActivation};

#[allow(unused)]
pub use network_layer::{DenseLayer, SpareLayer, CustomLayer};