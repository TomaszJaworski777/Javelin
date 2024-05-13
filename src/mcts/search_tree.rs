use crate::{core::Move, mcts::GameResult, options::Options};
use colored::*;
use std::ops::{Index, IndexMut};

use super::{node::Node, phantom_node::PhantomNode};

#[derive(Clone)]
pub struct SearchTree {
    tree: Vec<Node>,
    root: PhantomNode,
    tree_capacity: usize,
}
impl SearchTree {
    pub fn new() -> Self {
        Self {
            tree: Vec::new(),
            root: PhantomNode::new(0, Move::NULL, 0.0),
            tree_capacity: Options::get("Hash").get_value::<usize>() * 1024 * 1024 / (std::mem::size_of::<Node>() * 8),
        }
    }

    pub fn push(&mut self, node: &Node) -> i32 {
        self.tree.push(node.clone());
        (self.tree.len() - 1) as i32
    }

    pub fn node_count(&self) -> u32 {
        self.tree.len() as u32
    }

    pub fn capacity(&self) -> usize {
        self.tree_capacity
    }

    pub fn usage(&self) -> f32 {
        self.node_count() as f32 / self.tree_capacity as f32
    }

    pub fn child(&self, node_index: i32, child_index: usize) -> &PhantomNode {
        if node_index == -1 {
            &self.root
        } else {
            &self[node_index].children()[child_index]
        }
    }

    pub fn child_mut(&mut self, node_index: i32, child_index: usize) -> &mut PhantomNode {
        if node_index == -1 {
            &mut self.root
        } else {
            &mut self[node_index].children_mut()[child_index]
        }
    }

    pub fn get_best_phantom(&self) -> &PhantomNode {
        self.get_best_child_for_node(0)
    }

    pub fn get_pv_line(&self) -> String {
        let mut pv_line: Vec<String> = Vec::new();
        let mut phantom_node = self.get_best_child_for_node(0);
        pv_line.push(phantom_node.mv().to_string());

        while !self[phantom_node.index()].children().is_empty() {
            phantom_node = self.get_best_child_for_node(phantom_node.index());
            pv_line.push(phantom_node.mv().to_string());
        }

        pv_line.join(" ")
    }

    fn get_best_child_for_node(&self, node_index: i32) -> &PhantomNode {
        let mut best_node = &self.root;
        let mut best_score = f32::MIN;

        for child_phantom in self[node_index].children() {
            if child_phantom.visits() == 0 {
                continue;
            }

            if child_phantom.avg_score() > best_score {
                best_score = child_phantom.avg_score();
                best_node = child_phantom;
            }
        }

        best_node
    }

    #[allow(unused)]
    pub fn draw_tree_from_root(&self, max_depth: i32) {
        self.print_tree_usage();
        if !self.tree.is_empty() {
            self.draw_tree(&self.root, "".to_string(), false, true, max_depth, 0, 0.0, 0.0, false);
        }
    }

    #[allow(unused)]
    pub fn draw_tree_from_node(&self, node_index: i32, max_depth: i32) {
        let (node_phantom, node_depth) = self.find_node_phantom(node_index);
        self.print_tree_usage();
        if !self.tree.is_empty() {
            self.draw_tree(&node_phantom, "".to_string(), false, true, max_depth, node_depth, 0.0, 0.0, false);
        }
    }

    fn print_tree_usage(&self) {
        let usage_text = format!("{:.2}%", self.usage());
        println!(
            "Tree usage: {}/{} ({})",
            convert_number_memory_string(self.node_count()),
            convert_number_memory_string(self.capacity() as u32),
            heat_color(usage_text.as_str(), 1.0 - self.usage(), 0.0, 1.0)
        );
    }

    fn draw_tree(
        &self,
        phantom_node: &PhantomNode,
        prefix: String,
        last: bool,
        is_root: bool,
        max_depth: i32,
        current_depth: u32,
        heat_min_value: f32,
        heat_max_value: f32,
        has_promotion: bool,
    ) {
        if max_depth < 0 {
            return;
        }

        let new_prefix = if last { "    ".to_string() } else { "│   ".to_string() };
        let connector = if last { "└─> " } else { "├─> " };

        let prefix_string = prefix.clone() + connector;
        let game_result =
            if phantom_node.index() != -1 { self[phantom_node.index()].result() } else { GameResult::None };
        phantom_node.print_node(
            if is_root { "" } else { prefix_string.as_str() },
            is_root,
            heat_min_value,
            heat_max_value,
            has_promotion,
            game_result,
        );

        if max_depth == 0 || phantom_node.visits() == 0 {
            return;
        }

        let children = self[phantom_node.index()].children();
        let mut heat_min_value = f32::MAX;
        let mut heat_max_value = f32::MIN;
        let mut has_promotion = false;
        for child_phantom in children {
            heat_min_value = heat_min_value.min(child_phantom.policy());
            heat_max_value = heat_max_value.max(child_phantom.policy());
            if child_phantom.mv().is_promotion() {
                has_promotion = true;
            }
        }
        for (i, child_phantom) in children.iter().enumerate() {
            let is_last_child = i == children.len() - 1;
            self.draw_tree(
                child_phantom,
                prefix.clone() + if is_root { "" } else { &new_prefix },
                is_last_child,
                false,
                max_depth - 1,
                current_depth + 1,
                heat_min_value,
                heat_max_value,
                has_promotion,
            );
        }
    }

    fn find_node_phantom(&self, node_index: i32) -> (PhantomNode, u32) {
        self.find_node_phantom_step(node_index, &self.root)
    }

    fn find_node_phantom_step<'a>(
        &self,
        target_node_index: i32,
        phantom_to_process: &'a PhantomNode,
    ) -> (PhantomNode, u32) {
        if phantom_to_process.index() == target_node_index {
            return (*phantom_to_process, 0);
        }

        if phantom_to_process.visits() == 0 || self[phantom_to_process.index()].is_terminal() {
            return (self.root, 0);
        }

        for child_phantom in self[phantom_to_process.index()].children() {
            let result = self.find_node_phantom_step(target_node_index, child_phantom);
            if result.0 != self.root {
                return result;
            }
        }

        return (self.root, 0);
    }
}

impl Index<i32> for SearchTree {
    type Output = Node;

    fn index(&self, index: i32) -> &Self::Output {
        &self.tree[index as usize]
    }
}

impl IndexMut<i32> for SearchTree {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.tree[index as usize]
    }
}

fn convert_number_memory_string(number: u32) -> String {
    let byte_count = number as usize * std::mem::size_of::<Node>() * 8;
    if byte_count < 1024 {
        format!("{}B", byte_count).truecolor(192, 210, 255).to_string()
    } else if byte_count < (1024.0 * 1023.99) as usize {
        format!("{:.1}KB", byte_count as f32 / 1024.0).truecolor(192, 210, 255).to_string()
    } else if byte_count < (1024.0 * 1024.0 * 1023.99) as usize {
        format!("{:.1}MB", byte_count as f32 / (1024.0 * 1024.0)).truecolor(192, 210, 255).to_string()
    } else {
        format!("{:.1}GB", byte_count as f32 / (1024.0 * 1024.0 * 1024.0)).truecolor(192, 210, 255).to_string()
    }
}

fn heat_color(content: &str, value: f32, min_value: f32, max_value: f32) -> String {
    let scalar = (value - min_value) / (max_value - min_value);
    let r = (255.0 * (1.0 - scalar)) as u8;
    let g = (255.0 * scalar) as u8;
    content.truecolor(r, g, if r < 100 || g < 100 { 10 } else { 0 }).to_string()
}
