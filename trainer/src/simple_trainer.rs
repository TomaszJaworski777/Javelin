use std::{path::Path, process::Command};

use tch::nn::{seq, Sequential, VarStore};

pub struct SimpleTrainer<'a>{
    var_store: VarStore,
    net_structure: Sequential,
    name: &'a str,
    start_learning_rate: f32,
    learning_rate_drop: f32,
    drop_delay: u8,
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
            learning_rate_drop: 0.1,
            drop_delay: 5,
            export_path
        }
    }

    pub fn add_structure(&mut self, structure: Sequential) {
        self.net_structure = structure;
    }

    pub fn chnage_learning_rate(&mut self, start_lr: f32, drop_lr: f32, drop_delay: u8) {
        self.start_learning_rate = start_lr;
        self.learning_rate_drop = drop_lr;
        self.drop_delay = drop_delay;
    }

    pub fn build(&mut self) {
        let path = SimpleTrainer::TRAINING_PATH.to_string() + self.name + ".ot";
        if Path::new(&path).exists() {
            self.var_store.load(path).expect("Failed to load net training data!");
        }
    }

    pub fn run(&self) {

    }

    fn print_raport(&self) {

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