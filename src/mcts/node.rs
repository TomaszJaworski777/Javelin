use std::ops::Range;

use crate::core_structs::Move;

#[derive(Clone, Copy)]
pub struct Node {
    pub index: u32,
    pub total_value: f32,
    pub visit_count: u32,
    pub first_child_index: u32,
    pub children_count: u32,
    pub policy_value: f32,
    pub is_terminal: bool,
    pub _move: Move
}
impl Node 
{
    pub fn new(_move: Move) -> Self{
        Self{
            index: 0,
            total_value: 0.0,
            visit_count: 0,
            first_child_index: 0,
            children_count: 0,
            policy_value: 0.0,
            is_terminal: false,
            _move: _move
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children_count == 0
    }

    pub fn children(&self) -> Range<u32> {
        let end_index = self.first_child_index + self.children_count;
        self.first_child_index..end_index
    }

    pub fn avg_value(&self) -> f32 {
        if self.visit_count == 0{
            return 0.5;
        }
        self.total_value / self.visit_count as f32
    }

    pub fn print_node(&self, prefix: &str) {
        let move_str = if self._move == Move::NULL { "root".to_string() } else { format!("{}. {}", self.index, self._move.to_string()) };
        println!("{}{} Q({:.2}%) N({}) P({:.2}%)",
            prefix,
            move_str,
            self.avg_value() * 100.0,
            self.visit_count,
            self.policy_value * 100.0
        );
    }
}