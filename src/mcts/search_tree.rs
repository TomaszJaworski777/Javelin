use crate::{core::Move, mcts::GameResult, options::Options};
use colored::*;
use std::ops::{Index, IndexMut};

use super::{node::Node, phantom_node::PhantomNode};

#[derive(Clone)]
pub struct SearchTree {
    tree: Vec<Node>,
    root_phantom: PhantomNode,
    root_index: i32,
    empty_node_index: i32,
    used_nodes_count: usize,
    lru_head: i32,
    lru_tail: i32,
}
impl SearchTree {
    pub fn new() -> Self {
        let tree_capacity = Options::get("Hash").get_value::<usize>() * 1024 * 1024 / (std::mem::size_of::<Node>() * 8);
        let mut tree = Self {
            tree: vec![Node::new(GameResult::None, -1, 0); tree_capacity],
            root_phantom: PhantomNode::new(0, Move::NULL, 0.0),
            root_index: -1,
            empty_node_index: 0,
            used_nodes_count: 0,
            lru_head: -1,
            lru_tail: -1,
        };

        //Initialize linked list in the tree for replacing
        //nodes that weren't used recently, when tree is full
        let end_index = tree.capacity() as i32 - 1;

        for index in 0..end_index {
            tree[index].set_forward_link(index + 1);
        }

        tree
    }

    pub fn push(&mut self, node: Node) -> i32 {
        let mut new_node_index = self.empty_node_index;

        //New node index being equal to -1 means there is no more
        //space in the tree and we have to remove a node
        if new_node_index == -1 {
            new_node_index = self.lru_tail;
            let parent_index = self[new_node_index].parent();
            let child_index = self[new_node_index].child();

            self.child_mut(parent_index, child_index).set_index(-1);

            self.delete_node(new_node_index);
        }

        assert_ne!(new_node_index, -1);

        self.used_nodes_count += 1;
        self.empty_node_index = self[self.empty_node_index].forward_link();
        self[new_node_index] = node;

        self.append_to_lru(new_node_index);

        if self.used_nodes_count == 1 {
            self.lru_tail = new_node_index;
        }

        new_node_index
    }

    pub fn delete_node(&mut self, node_index: i32) {
        self.remove_from_lru(node_index);
        self[node_index].clear();

        let empty_node_index = self.empty_node_index;
        self[node_index].set_forward_link(empty_node_index);

        self.empty_node_index = node_index;
        self.used_nodes_count -= 1;
        assert!(self.used_nodes_count < self.capacity());
    }

    pub fn make_recently_used(&mut self, node_index: i32) {
        self.remove_from_lru(node_index);
        self.append_to_lru(node_index);
    }

    fn append_to_lru(&mut self, node_index: i32) {
        let old_head = self.lru_head;
        if old_head != -1 {
            self[old_head].set_backward_link(node_index);
        }
        self.lru_head = node_index;
        self[node_index].set_forward_link(old_head);
        self[node_index].set_backward_link(-1);
    }

    fn remove_from_lru(&mut self, node_index: i32) {
        let backward_link = self[node_index].backward_link_link();
        let forward_link = self[node_index].forward_link();

        if backward_link != -1 {
            self[backward_link].set_forward_link(forward_link);
        } else {
            self.lru_head = forward_link;
        }

        if forward_link != -1 {
            self[forward_link].set_backward_link(backward_link);
        } else {
            self.lru_tail = backward_link;
        }

        self[node_index].set_backward_link(-1);
        self[node_index].set_forward_link(-1);
    }

    pub fn node_count(&self) -> usize {
        self.used_nodes_count
    }

    pub fn capacity(&self) -> usize {
        self.tree.len()
    }

    pub fn usage(&self) -> f32 {
        self.node_count() as f32 / self.capacity() as f32
    }

    pub fn root_index(&self) -> i32 {
        self.root_index
    }

    pub fn set_root_index(&mut self, new_value: i32) {
        self.root_index = new_value
    }

    pub fn child(&self, node_index: i32, child_index: usize) -> &PhantomNode {
        if node_index == -1 {
            &self.root_phantom
        } else {
            &self[node_index].children()[child_index]
        }
    }

    pub fn child_mut(&mut self, node_index: i32, child_index: usize) -> &mut PhantomNode {
        if node_index == -1 {
            &mut self.root_phantom
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

        while (phantom_node.index() as usize) < self.capacity() && !self[phantom_node.index()].children().is_empty() {
            phantom_node = self.get_best_child_for_node(phantom_node.index());
            pv_line.push(phantom_node.mv().to_string());
        }

        pv_line.join(" ")
    }

    fn get_best_child_for_node(&self, node_index: i32) -> &PhantomNode {
        let mut best_node = &self.root_phantom;
        let mut best_score = f32::NEG_INFINITY;

        for child_phantom in self[node_index].children() {
            let score = if child_phantom.visits() == 0 {
                f32::NEG_INFINITY
            } else {
                let child_index = child_phantom.index();
                if child_index != -1 {
                    match self[child_index].result() {
                        GameResult::None => child_phantom.avg_score(),
                        GameResult::Draw => 0.5,
                        GameResult::Lose(n) => 1.0 + f32::from(n),
                        GameResult::Win(n) => f32::from(n) - 256.0,
                    }
                } else {
                    child_phantom.avg_score()
                }
            };

            if score > best_score {
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
            self.draw_tree(&self.root_phantom, "".to_string(), false, true, max_depth, 0, 0.0, 0.0, false);
        }
    }

    #[allow(unused)]
    pub fn draw_tree_from_node(&self, node_index: i32, max_depth: i32) {
        if node_index == -1 {
            return;
        }

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
            convert_number_memory_string(self.capacity()),
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
        self.find_node_phantom_step(node_index, &self.root_phantom)
    }

    fn find_node_phantom_step<'a>(
        &self,
        target_node_index: i32,
        phantom_to_process: &'a PhantomNode,
    ) -> (PhantomNode, u32) {
        if phantom_to_process.index() == -1 {
            return (self.root_phantom, 0);
        }

        if phantom_to_process.index() == target_node_index {
            return (*phantom_to_process, 0);
        }

        if phantom_to_process.visits() == 0 || self[phantom_to_process.index()].is_terminal() {
            return (self.root_phantom, 0);
        }

        for child_phantom in self[phantom_to_process.index()].children() {
            let result = self.find_node_phantom_step(target_node_index, child_phantom);
            if result.0 != self.root_phantom {
                return result;
            }
        }

        return (self.root_phantom, 0);
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

fn convert_number_memory_string(number: usize) -> String {
    let byte_count = number * std::mem::size_of::<Node>() * 8;
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
