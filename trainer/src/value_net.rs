use javelin::ValueNetwork;
use std::{fmt::Debug, fs::File, io::Write, mem, path::Path};

use tch::Tensor;

use crate::core_net_struct::SimpleNet;

pub struct ValueNet {
    pub net: SimpleNet,
}
impl ValueNet {
    pub const ARCHITECTURE: &[usize] = &[768, 64, 1];
    pub const NET_PATH: &str = "../resources/training/value.ot";
    pub const EXPORT_PATH: &str = "../resources/nets/value.net";

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

    pub fn export(&self) {
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

        let file = File::create(ValueNet::EXPORT_PATH);
        let struct_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                &value_network as *const ValueNetwork as *const u8,
                mem::size_of::<ValueNetwork>(),
            )
        };
        file.unwrap().write_all(struct_bytes).expect("Failed to write data!");
    }

    pub fn evaluate(&self, inputs: Vec<f32>) -> Tensor {
        self.net.evaluate(inputs).unwrap()
    }
}

fn vec_to_array_unchecked<T, const N: usize>(mut vec: Vec<T>) -> [T; N] {
    assert_eq!(vec.len(), N);
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec);
    unsafe { std::ptr::read(ptr as *const [T; N]) }
}

fn vec2d_to_array2d_unchecked<T, const N: usize, const M: usize>(vec: Vec<Vec<T>>) -> [[T; N]; M]
where
    T: Default + Debug, // Default can help in handling partially initialized arrays in case of panic
{
    let mut temp_storage: Vec<[T; N]> = Vec::with_capacity(M);
    for inner_vec in vec {
        let msg = format!("Inner vector does not have the correct length {}, {}", inner_vec.len(), N);
        assert_eq!(inner_vec.len(), N, "{msg}");
        let inner_array: [T; N] = inner_vec.try_into().unwrap(); // Can panic if sizes don't match
        temp_storage.push(inner_array);
    }

    temp_storage.try_into().unwrap()
}
