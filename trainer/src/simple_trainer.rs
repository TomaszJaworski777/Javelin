use std::{fs::File, io::Write, path::Path, process::Command, time::Instant};

use datagen::Files;
use javelin::{PolicyNetwork, ValueNetwork};
use tch::{nn::{seq, Module, Optimizer, OptimizerConfig, Sequential, VarStore}, Kind};
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::{policy_data_loader::PolicyDataLoader, value_data_loader::ValueDataLoader};

pub struct SimpleTrainer<'a>{
    pub var_store: VarStore,
    net_structure: Sequential,
    name: &'a str,
    start_learning_rate: f64,
    learning_rate_drop: f64,
    drop_delay: u8,
    batch_size: usize,
    epoch_count: u32,
    export_path: String
}
impl<'a> SimpleTrainer<'a> {
    const TRAINING_PATH: &'static str = "../../resources/training/";
    const EXPORT_PATH: &'static str = "../../resources/nets/";
    const CHECKPOINT_PATH: &'static str = "../../resources/training/checkpoints/";

    pub fn new(name: &'a str) -> Self {
        let var_store = VarStore::new(tch::Device::Cpu);
        let export_path = SimpleTrainer::EXPORT_PATH.to_string() + name + ".net";
        Self {
            var_store,
            net_structure: seq(),
            name,
            start_learning_rate: 0.001,
            learning_rate_drop: 0.5,
            drop_delay: 5,
            batch_size: 16384,
            epoch_count: 400,
            export_path
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

    pub fn build(&mut self) {
        let path = SimpleTrainer::TRAINING_PATH.to_string() + self.name + ".ot";
        if Path::new(&path).exists() {
            self.var_store.load(path).expect("Failed to load net training data!");
        }
    }

    pub fn run<const VALUE: bool>(&self) {
        let mut train_data = Files::new();
        let _ = train_data.load();

        let mut optimizer = tch::nn::AdamW::default().build(&self.var_store, self.start_learning_rate).unwrap();

        if VALUE {
            self.print_search_params(train_data.value_data.len());
            self.value_run(&mut optimizer, &train_data);
        } else {
            self.print_search_params(train_data.policy_data.len());
            self.policy_run(&mut optimizer, &train_data);
        }
    }

    fn value_run(&self, optimizer: &mut Optimizer, train_data: &Files){
        let mut current_learning_rate = self.start_learning_rate;
        let mut lowest_loss = 0.0;
        let mut loss_delay = 0u8;
        
        let mut batches = ValueDataLoader::get_batches(&train_data.value_data, self.batch_size);
        println!("Finished preparing data!");
        
        let timer = Instant::now();
        for epoch in 0..self.epoch_count {
            let mut total_loss = 0.0;
            batches.shuffle(&mut thread_rng());

            for (inputs, targets) in &batches {
                let outputs = self.net_structure.forward(&inputs);
                let loss = (outputs - targets).pow_tensor_scalar(2).sum(Kind::Float).divide_scalar(self.batch_size as f64);
    
                total_loss += loss.double_value(&[]) as f32 / batches.len() as f32;
                optimizer.backward_step(&loss);
            }

            println!("epoch {} time {:.2} loss {:.5} lr {:.7}",
                epoch,
                timer.elapsed().as_secs_f32(),
                total_loss,
                current_learning_rate
            );

            if total_loss >= lowest_loss {
                loss_delay += 1;
            } else {
                loss_delay -= 1;
                loss_delay = loss_delay.clamp(0, u8::MAX);
                lowest_loss = total_loss;
            }

            if loss_delay >= self.drop_delay {
                loss_delay = 0;
                current_learning_rate *= self.learning_rate_drop;
                optimizer.set_lr(current_learning_rate);
            }

            self.var_store.save(SimpleTrainer::TRAINING_PATH.to_string() + self.name + ".ot").expect("Failed to save training progress!");
            export_value(&self.var_store, &self.export_path, [768,4,1]);
            let checkpoint_path = SimpleTrainer::CHECKPOINT_PATH.to_string() + format!("{}-epoch{}.net", self.name, epoch).as_str();
            export_value(&self.var_store, &checkpoint_path, [768,4,1]);
        }
    }

    fn policy_run(&self, optimizer: &mut Optimizer, train_data: &Files){
        let mut current_learning_rate = self.start_learning_rate;
        let mut lowest_loss = 0.0;
        let mut loss_delay = 0u8;
        
        let mut batches = PolicyDataLoader::get_batches(&train_data.policy_data, self.batch_size);
        println!("Finished preparing data!");
        
        let timer = Instant::now();
        for epoch in 1..=self.epoch_count {
            let mut total_loss = 0.0;
            batches.shuffle(&mut thread_rng());

            for (inputs, targets, mask, negative) in &batches {
                let outputs = &self.net_structure.forward(&inputs).multiply(mask).g_add(negative).softmax(-1, Kind::Float);
                let loss = (outputs - targets).pow_tensor_scalar(2).sum(Kind::Float).divide_scalar(self.batch_size as f64);
                total_loss += loss.double_value(&[]) as f32 / batches.len() as f32;
                optimizer.backward_step(&loss);
            }

            println!("epoch {} time {:.2} loss {:.5} lr {:.7}",
                epoch,
                timer.elapsed().as_secs_f32(),
                total_loss,
                current_learning_rate
            );

            if total_loss >= lowest_loss {
                loss_delay += 1;
            } else {
                loss_delay -= 1;
                loss_delay = loss_delay.clamp(0, u8::MAX);
                lowest_loss = total_loss;
            }

            if loss_delay >= self.drop_delay {
                loss_delay = 0;
                current_learning_rate *= self.learning_rate_drop;
                optimizer.set_lr(current_learning_rate);
            }

            self.var_store.save(SimpleTrainer::TRAINING_PATH.to_string() + self.name + ".ot").expect("Failed to save training progress!");
            export_policy(&self.var_store, &self.export_path, [768,384]);
            let checkpoint_path = SimpleTrainer::CHECKPOINT_PATH.to_string() + format!("{}-epoch{}.net", self.name, epoch).as_str();
            export_policy(&self.var_store, &checkpoint_path, [768,384]);
        }
    }


    fn print_search_params(&self, data_size: usize) {
        println!("Starting learning");
        println!("  Net name:      {}", self.name);
        println!("  Data size:     {}", data_size);
        println!("  Batch size:    {}", self.batch_size);
        println!("  Epoch count:   {}", self.epoch_count);
        println!("  Learning rate: {}", self.start_learning_rate);
        println!("  Learning drop: {}", self.learning_rate_drop);
        println!("  Drop delay:    {}", self.drop_delay);
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

fn export_value(var_store: &VarStore, path: &str, architecture: [usize; 3]){
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

fn export_policy(var_store: &VarStore, path: &str, architecture: [usize; 2]){
    let mut policy_network = boxed_and_zeroed::<PolicyNetwork>();
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
            policy_network.set_layer_weights(index, weights);
        } else {
            let length = architecture[1 + index];
            let mut biases = vec![0.0; length];
            for output_index in 0..length {
                biases[output_index] = tensor.double_value(&[output_index as i64]) as f32;
            }
            policy_network.set_layer_biases(index, biases);
        }
    }

    let file = File::create(path);
    let size = std::mem::size_of::<PolicyNetwork>();
    unsafe {
        let slice: *const u8 = std::slice::from_ref(policy_network.as_ref()).as_ptr().cast();
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