use javelin::ValueNetwork;
use std::{fs::File, io::Write, mem, path::Path};

use tch::Tensor;

use crate::core_net_struct::SimpleNet;

pub struct ValueNet {
    pub net: SimpleNet,
}
impl ValueNet {
    pub const ARCHITECTURE: &'static [usize] = &[768, 64, 1];
    pub const NET_PATH: &'static str = "../resources/training/value.ot";
    pub const EXPORT_PATH: &'static str = "../resources/nets/value-000.net";

    pub fn new() -> Self {
        let mut net = SimpleNet::new(ValueNet::ARCHITECTURE);
        let path = Path::new(ValueNet::NET_PATH);
        if path.exists() {
            let _ = net.vs.load(ValueNet::NET_PATH);
        }
        Self { net }
    }

    pub fn save(&self) {
        let _ = self.net.save(ValueNet::NET_PATH);
    }

    pub fn export_final(&self) {
        self.export(ValueNet::EXPORT_PATH);
    }

    pub fn export(&self, path: &str) {
        let mut value_network = ValueNetwork::new();
        for (name, tensor) in self.net.vs.variables() {
            let name_split: Vec<&str> = name.split(".").collect();
            let index = name_split[0].parse::<usize>().expect("Incorrect index!");
            if name_split[1] == "weight" {
                let input_length = ValueNet::ARCHITECTURE[0 + index];
                let output_length = ValueNet::ARCHITECTURE[1 + index];
                let mut weights = vec![vec![0.0; input_length]; output_length];
                for weight_index in 0..input_length {
                    for output_index in 0..output_length {
                        weights[output_index][weight_index] =
                            tensor.get(output_index as i64).double_value(&[weight_index as i64]) as f32;
                    }
                }
                value_network.set_layer_weights(index, weights);
            } else {
                let length = ValueNet::ARCHITECTURE[1 + index];
                let mut biases = vec![0.0; length];
                for output_index in 0..length {
                    biases[output_index] = tensor.double_value(&[output_index as i64]) as f32;
                }
                value_network.set_layer_biases(index, biases);
            }
        }

        let file = File::create(path);
        let struct_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                &value_network as *const ValueNetwork as *const u8,
                mem::size_of::<ValueNetwork>(),
            )
        };
        file.unwrap().write_all(struct_bytes).expect("Failed to write data!");
    }

    pub fn evaluate(&self, inputs: &Vec<f32>) -> Tensor {
        self.net.evaluate(&inputs)
    }
}
