use std::ops::{Index, IndexMut, Range};

use crate::core_structs::Move;

pub struct SearchTree(Vec<Node>);
impl SearchTree {
    pub fn new() -> Self{
        Self(Vec::new())
    }

    pub fn get_best_node<const RAPORT: bool>(&self) -> &Node{
        let mut best_node = &self[0];
        let mut best_score = f32::MIN;

        for child_index in best_node.children() {
            let child = &self[child_index];

            if child.avg_value() > best_score {
                best_score = child.avg_value();
                best_node = child;
            }
        }

        if RAPORT {
            self.draw_tree_from_root(1);
        }
        best_node
    }

    pub fn draw_tree_from_root(&self, max_depth: i32) {
        if !self.0.is_empty() {
            self.draw_tree(0, "".to_string(), false, true, max_depth);
        }
    }

    pub fn draw_tree_from_node(&self, node_index: u32, max_depth: i32) {
        if !self.0.is_empty() {
            self.draw_tree(node_index, "".to_string(), false, true, max_depth);
        }
    }

    fn draw_tree(&self, node_index: u32, prefix: String, last: bool, is_root: bool, max_depth: i32) {
        if max_depth < 0 {
            return;
        }

        let node = self[node_index];
        let new_prefix = if last { "    ".to_string() } else { "│   ".to_string() };
        let connector = if last { "└─> " } else { "├─> " };

        let prefix_string = prefix.clone() + connector;
        node.print_node(if is_root { "" } else { prefix_string.as_str() });

        if max_depth == 0 {
            return;
        }

        let children = node.children();
        let children_count = children.end - children.start;
        for (i, child_index) in children.enumerate() {
            let is_last_child = i as u32 == children_count - 1;
            self.draw_tree(child_index, prefix.clone() + if is_root { "" } else { &new_prefix }, is_last_child, false, max_depth - 1);
        }
    }
}

impl Index<u32> for SearchTree {
    type Output = Node;

    fn index(&self, index: u32) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u32> for SearchTree {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

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
        self.children_count == 0 && !self.is_terminal
    }

    pub fn children(&self) -> Range<u32> {
        let end_index = self.first_child_index + self.children_count;
        self.first_child_index..end_index
    }

    pub fn avg_value(&self) -> f32 {
        self.total_value / (self.visit_count as f32).max(1.0)
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