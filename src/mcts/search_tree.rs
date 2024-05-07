use crate::core::Board;
use colored::*;
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

            if child.visit_count == 0 {
                continue;
            }

            if child.avg_value() > best_score {
                best_score = child.avg_value();
                best_node = child;
            }
        }

        best_node
    }

    #[allow(unused)]
    pub fn draw_tree_from_root(&self, max_depth: i32, board: &Board) {
        let usage_percentage = self.0.len() as f32 / u32::MAX as f32;
        let usage_text = format!("{:.2}%", usage_percentage);
        println!(
            "Tree usage: {}/{} ({})",
            convert_number_memory_string(self.0.len() as u32),
            convert_number_memory_string(u32::MAX),
            heat_color(usage_text.as_str(), 1.0 - usage_percentage, 0.0, 1.0)
        );

        if !self.0.is_empty() {
            self.draw_tree(0, "".to_string(), false, true, max_depth, 0, &board, 0.0, 0.0, false);
        }
    }

    #[allow(unused)]
    pub fn draw_tree_from_node(&self, node_index: u32, max_depth: i32, board: &Board) {
        let usage_percentage = self.0.len() as f32 / u32::MAX as f32;
        let usage_text = format!("{:.2}%", usage_percentage);
        println!(
            "Tree usage: {}/{} ({})",
            convert_number_memory_string(self.0.len() as u32),
            convert_number_memory_string(u32::MAX),
            heat_color(usage_text.as_str(), 1.0 - usage_percentage, 0.0, 1.0)
        );

        if !self.0.is_empty() {
            self.draw_tree(
                node_index,
                "".to_string(),
                false,
                true,
                max_depth,
                self.depth_of_node(node_index).unwrap(),
                &board,
                0.0,
                0.0,
                false,
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
        heat_min_value: f32,
        heat_max_value: f32,
        has_promotion: bool,
    ) {
        if max_depth < 0 {
            return;
        }

        let node = self[node_index];
        let new_prefix = if last { "    ".to_string() } else { "│   ".to_string() };
        let connector = if last { "└─> " } else { "├─> " };

        let prefix_string = prefix.clone() + connector;
        node.print_node(
            if is_root { "" } else { prefix_string.as_str() },
            is_root,
            heat_min_value,
            heat_max_value,
            has_promotion,
        );

        if max_depth == 0 {
            return;
        }

        let children = node.children();
        let children_count = children.end - children.start;
        let mut heat_min_value = f32::MAX;
        let mut heat_max_value = f32::MIN;
        let mut has_promotion = false;
        for child_index in children.clone() {
            let child = self[child_index];
            heat_min_value = heat_min_value.min(child.policy_value);
            heat_max_value = heat_max_value.max(child.policy_value);
            if child.mv.is_promotion() {
                has_promotion = true;
            }
        }
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
                heat_min_value,
                heat_max_value,
                has_promotion,
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

fn convert_number_memory_string(number: u32) -> String {
    let byte_count = number as usize * std::mem::size_of::<Node>();
    if byte_count < 1000 {
        format!("{}B", byte_count).truecolor(192, 210, 255).to_string()
    } else if byte_count < 1_000_000 {
        format!("{:.1}KB", byte_count as f32 / 1000.0).truecolor(192, 210, 255).to_string()
    } else if byte_count < 1_000_000_000 {
        format!("{:.1}MB", byte_count as f32 / 1_000_000.0).truecolor(192, 210, 255).to_string()
    } else {
        format!("{:.1}GB", byte_count as f32 / 1_000_000_000.0).truecolor(192, 210, 255).to_string()
    }
}

fn heat_color(content: &str, value: f32, min_value: f32, max_value: f32) -> String {
    let scalar = (value - min_value) / (max_value - min_value);
    let r = (255.0 * (1.0 - scalar)) as u8;
    let g = (255.0 * scalar) as u8;
    content.truecolor(r, g, if r < 100 || g < 100 { 10 } else { 0 }).to_string()
}
