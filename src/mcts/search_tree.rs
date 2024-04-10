use crate::core::Board;
use std::ops::{Index, IndexMut};

use super::node::Node;

#[derive(Clone)]
pub struct SearchTree(Vec<Node>);
impl SearchTree {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, node: &Node) {
        self.0.push(*node);
    }

    pub fn node_count(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn get_best_node(&self) -> &Node {
        self.get_best_child_for_node(0)
    }

    pub fn get_pv_line(&self) -> String {
        let mut pv_line: Vec<String> = Vec::new();
        let mut current_best_node = self.get_best_child_for_node(0);
        pv_line.push(current_best_node.mv.to_string());

        while !current_best_node.is_leaf() {
            current_best_node = self.get_best_child_for_node(current_best_node.index);
            pv_line.push(current_best_node.mv.to_string());
        }

        pv_line.join(" ")
    }

    fn get_best_child_for_node(&self, node_index: u32) -> &Node {
        let mut best_node = &self[node_index];
        let mut best_score = f32::MIN;

        for child_index in self[node_index].children() {
            let child = &self[child_index];

            if child.avg_value() > best_score {
                best_score = child.avg_value();
                best_node = child;
            }
        }

        best_node
    }

    #[allow(unused)]
    pub fn draw_tree_from_root(&self, max_depth: i32, board: &Board) {
        if !self.0.is_empty() {
            self.draw_tree(0, "".to_string(), false, true, max_depth, 0, &board);
        }
    }

    #[allow(unused)]
    pub fn draw_tree_from_node(&self, node_index: u32, max_depth: i32, board: &Board) {
        if !self.0.is_empty() {
            self.draw_tree(
                node_index,
                "".to_string(),
                false,
                true,
                max_depth,
                self.depth_of_node(node_index).unwrap(),
                &board,
            );
        }
    }

    fn draw_tree(
        &self,
        node_index: u32,
        prefix: String,
        last: bool,
        is_root: bool,
        max_depth: i32,
        current_depth: u32,
        board: &Board,
    ) {
        if max_depth < 0 {
            return;
        }

        let node = self[node_index];
        let new_prefix = if last { "    ".to_string() } else { "│   ".to_string() };
        let connector = if last { "└─> " } else { "├─> " };

        let prefix_string = prefix.clone() + connector;
        node.print_node(if is_root { "" } else { prefix_string.as_str() }, current_depth % 2 == 0);

        if max_depth == 0 {
            return;
        }

        let children = node.children();
        let children_count = children.end - children.start;
        for (i, child_index) in children.enumerate() {
            let is_last_child = i as u32 == children_count - 1;
            self.draw_tree(
                child_index,
                prefix.clone() + if is_root { "" } else { &new_prefix },
                is_last_child,
                false,
                max_depth - 1,
                current_depth + 1,
                &board,
            );
        }
    }

    fn depth_of_node(&self, target_index: u32) -> Option<u32> {
        self.depth_of_node_recursive(target_index, 0, 0)
    }

    fn depth_of_node_recursive(&self, target_index: u32, current_index: u32, current_depth: u32) -> Option<u32> {
        if current_index == target_index {
            return Some(current_depth);
        }

        let node = self[current_index];
        for child_index in node.children() {
            if let Some(depth) = self.depth_of_node_recursive(target_index, child_index, current_depth + 1) {
                return Some(depth);
            }
        }

        None
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
