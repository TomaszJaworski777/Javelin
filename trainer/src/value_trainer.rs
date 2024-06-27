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
    drop_delay: u32,
    batch_size: usize,
    batches_per_superbatch: usize,
    superbach_count: u32,
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
            learning_rate_drop: 0.1,
            drop_delay: 0,
            batch_size: 0,
            batches_per_superbatch: 0,
            superbach_count: 0,
        }
    }

    pub fn add_structure(&mut self, structure: Sequential) {
        self.net_structure = structure;
    }

    pub fn change_learning_rate(&mut self, start_lr: f64, drop_lr: f64, drop_delay: u32) {
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

    pub fn change_superbatch_count(&mut self, count: u32) {
        self.superbach_count = count;
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

        let mut superbatch_index = 0u32;
        let mut batch_index = 0;
        let mut total_loss: f32 = 0.0;
        let mut data_chunk_start_index = 0;

        'training: loop {
            let data_chunk_end_index =
                (data_chunk_start_index + 512 * self.batch_size).min(train_data.value_data.len());
            let value_data = ValueDataLoader::get_batches(
                &train_data.value_data[data_chunk_start_index..data_chunk_end_index].to_vec(),
                self.batch_size,
            );
            data_chunk_start_index = data_chunk_end_index % train_data.value_data.len();
            let timer = Instant::now();

            for (index, (inputs, targets)) in value_data.into_iter().enumerate() {
                let outputs = self.net_structure.forward(&inputs);
                let loss =
                    (outputs - targets).pow_tensor_scalar(2).sum(Kind::Float).divide_scalar(self.batch_size as f64);

                total_loss += loss.double_value(&[]) as f32;
                optimizer.backward_step(&loss);

                batch_index += 1;

                print!(
                    "> Superbatch {}/{} Batch {}/{} - {index} Speed {:.0}\r",
                    superbatch_index + 1,
                    self.superbach_count,
                    batch_index % self.batches_per_superbatch,
                    self.batches_per_superbatch,
                    (index * self.batch_size) as f32 / timer.elapsed().as_secs_f32()
                );
                let _ = std::io::stdout().flush();

                if batch_index % self.batches_per_superbatch == 0 {
                    superbatch_index += 1;
                    println!(
                        "> Superbatch {superbatch_index}/{} Running Loss {}",
                        self.superbach_count,
                        total_loss / self.batches_per_superbatch as f32
                    );
                    total_loss = 0.0;

                    if superbatch_index % self.drop_delay == 0 {
                        current_learning_rate *= self.learning_rate_drop;
                        optimizer.set_lr(current_learning_rate);
                        println!("Dropping LR to {current_learning_rate}");
                    }

                    self.var_store
                        .save(ValueTrainer::TRAINING_PATH.to_string() + self.name + ".ot")
                        .expect("Failed to save training progress!");
                    let checkpoint_path = ValueTrainer::CHECKPOINT_PATH.to_string()
                        + format!("{}-sb{}.net", self.name, superbatch_index).as_str();
                    export_value(&self.var_store, &checkpoint_path, [768, 128, 1]);

                    if superbatch_index == self.superbach_count {
                        break 'training;
                    }
                }
            }
        }

        loop {}
    }

    fn print_search_params(&self, data_size: usize) {
        let throughput = self.superbach_count as usize * self.batches_per_superbatch * self.batch_size;

        println!("Starting learning");
        println!("  Net name:           {}", self.name);
        println!("  Data size:          {}", data_size);
        println!("  Batch size:         {}", self.batch_size);
        println!("  Batches/superbatch: {}", self.batches_per_superbatch);
        println!("  Epoch count:        {:.2}", throughput as f64 / data_size as f64);
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
            let mut weights = vec![vec![0.0; output_length]; input_length];
            for input_index in 0..input_length {
                for output_index in 0..output_length {
                    weights[input_index][output_index] =
                        tensor.get(output_index as i64).double_value(&[input_index as i64]) as f32;
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
