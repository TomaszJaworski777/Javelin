use std::{fs::File, io::Write, path::Path, process::Command, time::Instant};

use crate::value_data_loader::ValueDataLoader;
use datagen::Files;
use javelin::ValueNetwork;
use tch::{
    nn::{seq, Module, OptimizerConfig, Sequential, VarStore},
    Kind,
};

pub struct ValueTrainer<'a> {
    pub var_store: VarStore,
    net_structure: Sequential,
    name: &'a str,
    start_learning_rate: f64,
    learning_rate_drop: f64,
    drop_delay: u8,
    batch_size: usize,
    batches_per_superbatch: usize,
    epoch_count: u32,
}
impl<'a> ValueTrainer<'a> {
    const TRAINING_PATH: &'static str = "../../resources/training/";
    const CHECKPOINT_PATH: &'static str = "../../resources/training/checkpoints/";

    pub fn new(name: &'a str) -> Self {
        let var_store = VarStore::new(tch::Device::Cpu);
        Self {
            var_store,
            net_structure: seq(),
            name,
            start_learning_rate: 0.001,
            learning_rate_drop: 0.5,
            drop_delay: 5,
            batch_size: 16384,
            batches_per_superbatch: 100,
            epoch_count: 400,
        }
    }

    pub fn add_structure(&mut self, structure: Sequential) {
        self.net_structure = structure;
    }

    pub fn change_learning_rate(&mut self, start_lr: f64, drop_lr: f64, drop_delay: u8) {
        self.start_learning_rate = start_lr;
        self.learning_rate_drop = drop_lr;
        self.drop_delay = drop_delay;
    }

    pub fn change_batch_size(&mut self, size: usize) {
        self.batch_size = size;
    }

    pub fn change_batch_per_superbatch_count(&mut self, count: usize) {
        self.batches_per_superbatch = count;
    }

    pub fn change_epoch_count(&mut self, count: u32) {
        self.epoch_count = count;
    }

    pub fn build(&mut self) {
        let path = ValueTrainer::TRAINING_PATH.to_string() + self.name + ".ot";
        if Path::new(&path).exists() {
            self.var_store.load(path).expect("Failed to load net training data!");
        }
    }

    pub fn run(&self) {
        let mut train_data = Files::new();
        let _ = train_data.load_value();
        let mut optimizer = tch::nn::AdamW::default().build(&self.var_store, self.start_learning_rate).unwrap();
        self.print_search_params(train_data.value_data.len());

        let mut current_learning_rate = self.start_learning_rate;

        let data_per_superbatch = self.batches_per_superbatch * self.batch_size;
        let superbatches_count = &train_data.value_data.len() / data_per_superbatch;

        let timer = Instant::now();
        for epoch in 1..=self.epoch_count {
            let mut total_loss = 0.0;

            for superbatch_index in 0..superbatches_count {
                let start_index = superbatch_index * data_per_superbatch;
                let end_index = start_index + data_per_superbatch;
                let batches = ValueDataLoader::get_batches(
                    &train_data.value_data[start_index..end_index].to_vec(),
                    self.batch_size,
                );
                for (inputs, targets) in &batches {
                    let outputs = self.net_structure.forward(&inputs);
                    let loss =
                        (outputs - targets).pow_tensor_scalar(2).sum(Kind::Float).divide_scalar(self.batch_size as f64);

                    total_loss += loss.double_value(&[]) as f32 / (superbatches_count * batches.len()) as f32;
                    optimizer.backward_step(&loss);
                }
            }

            println!(
                "epoch {} time {:.2} loss {:.5} lr {:.7}",
                epoch,
                timer.elapsed().as_secs_f32(),
                total_loss,
                current_learning_rate
            );

            if epoch != 0 && epoch % self.drop_delay as u32 == 0 {
                current_learning_rate *= self.learning_rate_drop;
                optimizer.set_lr(current_learning_rate);
            }

            self.var_store
                .save(ValueTrainer::TRAINING_PATH.to_string() + self.name + ".ot")
                .expect("Failed to save training progress!");
            let checkpoint_path =
                ValueTrainer::CHECKPOINT_PATH.to_string() + format!("{}-epoch{}.net", self.name, epoch).as_str();
            export_value(&self.var_store, &checkpoint_path, [768, 32, 1]);
        }
    }

    fn print_search_params(&self, data_size: usize) {
        println!("Starting learning");
        println!("  Net name:           {}", self.name);
        println!("  Data size:          {}", data_size);
        println!("  Batch size:         {}", self.batch_size);
        println!("  Batches/superbatch: {}", self.batches_per_superbatch);
        println!("  Superbatches/epoch: {}", data_size / (self.batches_per_superbatch * self.batch_size));
        println!("  Epoch count:        {}", self.epoch_count);
        println!("  Learning rate:      {}", self.start_learning_rate);
        println!("  Learning drop:      {}", self.learning_rate_drop);
        println!("  Drop delay:         {}", self.drop_delay);
        println!("Learning log:");
    }
}

#[allow(unused)]
fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear").spawn().expect("clear command failed to start").wait().expect("failed to wait");
    };
}

fn export_value(var_store: &VarStore, path: &str, architecture: [usize; 3]) {
    let mut value_network = boxed_and_zeroed::<ValueNetwork>();
    for (name, tensor) in var_store.variables() {
        let name_split: Vec<&str> = name.split(".").collect();
        let index = name_split[0].parse::<usize>().expect("Incorrect index!");
        if name_split[1] == "weight" {
            let input_length = architecture[0 + index];
            let output_length = architecture[1 + index];
            let mut weights = vec![vec![0.0; input_length]; output_length];
            for weight_index in 0..input_length {
                for output_index in 0..output_length {
                    weights[output_index][weight_index] =
                        tensor.get(output_index as i64).double_value(&[weight_index as i64]) as f32;
                }
            }
            value_network.set_layer_weights(index, weights);
        } else {
            let length = architecture[1 + index];
            let mut biases = vec![0.0; length];
            for output_index in 0..length {
                biases[output_index] = tensor.double_value(&[output_index as i64]) as f32;
            }
            value_network.set_layer_biases(index, biases);
        }
    }

    let file = File::create(path);
    let size = std::mem::size_of::<ValueNetwork>();
    unsafe {
        let slice: *const u8 = std::slice::from_ref(value_network.as_ref()).as_ptr().cast();
        let struct_bytes: &[u8] = std::slice::from_raw_parts(slice, size);
        file.unwrap().write_all(struct_bytes).expect("Failed to write data!");
    }
}

fn boxed_and_zeroed<T>() -> Box<T> {
    unsafe {
        let layout = std::alloc::Layout::new::<T>();
        let ptr = std::alloc::alloc_zeroed(layout);
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        Box::from_raw(ptr.cast())
    }
}
