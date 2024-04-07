use std::ops::{Index, IndexMut};
use super::node::Node;

pub struct SearchTree(Vec<Node>);
impl SearchTree {
    pub fn new() -> Self{
        Self(Vec::new())
    }

    pub fn push(&mut self, node: &Node) {
        self.0.push(*node);
    }

    pub fn node_count(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn get_best_node(&self) -> &Node{
        let mut best_node = &self[0];
        let mut best_score = f32::MIN;

        for child_index in best_node.children() {
            let child = &self[child_index];

            if child.avg_value() > best_score {
                best_score = child.avg_value();
                best_node = child;
            }
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