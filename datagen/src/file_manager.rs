use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;
use bytemuck::Pod;

use crate::structs::{ChessPolicyData, PieceBoard};

pub struct Files {
    pub value_data: Vec<PieceBoard>,
    pub policy_data: Vec<ChessPolicyData>,
}

#[allow(unused)]
impl Files {
    const VALUE_PATH: &'static str = "../resources/data/value.data";
    const POLICY_PATH: &'static str = "../resources/data/policy.data";

    pub fn new() -> Self {
        Self {
            value_data: Vec::new(),
            policy_data: Vec::new(),
        }
    }

    pub fn push_value(&mut self, board: &PieceBoard, filter: bool) -> bool{
        if !filter{
            self.value_data.push(*board);
            return true;
        }
        return false;
    }

    pub fn push_policy(&mut self, policy: &ChessPolicyData, filter: bool) -> bool{
        if !filter{
            self.policy_data.push(*policy);
            return true;
        }
        return false;
    }

    pub fn save(&self) -> io::Result<()> {
        self.save_data::<PieceBoard>(Self::VALUE_PATH, &self.value_data)?;
        self.save_data::<ChessPolicyData>(Self::POLICY_PATH, &self.policy_data)?;
        Ok(())
    }

    pub fn load(&mut self) -> io::Result<()> {
        self.value_data = self.load_data::<PieceBoard>(Self::VALUE_PATH)?;
        self.policy_data = self.load_data::<ChessPolicyData>(Self::POLICY_PATH)?;
        Ok(())
    }

    fn save_data<T: Pod>(&self, path: &str, data: &[T]) -> io::Result<()> {
        let bytes = bytemuck::cast_slice(data);
        fs::write(path, bytes)
    }

    fn load_data<T: Pod + Copy>(&self, path: &str) -> io::Result<Vec<T>> {
        let path = Path::new(path);
        if !path.exists() {
            return Ok(Vec::new());
        }
    
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
    
        if buffer.len() % std::mem::size_of::<T>() != 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Data is not aligned or incomplete"));
        }
    
        let chunks = buffer.chunks_exact(std::mem::size_of::<T>());
        Ok(chunks.map(|chunk| bytemuck::from_bytes::<T>(chunk)).copied().collect())
    }
}