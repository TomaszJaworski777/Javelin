mod activation;
mod network_layer;

#[allow(unused)]
pub use activation::{NoActivation, ReLUActivation, ScReLUActivation, SigmoidActivation};

#[allow(unused)]
pub use network_layer::{CustomLayer, DenseLayer, SpareLayer};
