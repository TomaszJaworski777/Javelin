use std::ops::Range;
use arrayvec::ArrayVec;
use crate::{board::Board, core_structs::{Move, MoveList}, eval::Evaluation, movegen::MoveProvider};

type SelectionHistory = ArrayVec<u32, 128>;

#[derive(Clone, Copy)]
struct Node {
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

    pub fn all_children_terminal(&self, search_tree: &SearchTree) -> bool {
        for child_index in self.children() {
            let child = search_tree.get_node(child_index);
            if !child.is_terminal {
                return false;
            }
        }
        true
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

struct SearchTree(Vec<Node>);
impl SearchTree {
    pub fn new() -> Self{
        Self(Vec::new())
    }

    pub fn get_node(&self, index: u32) -> Node {
        self.0[index as usize]
    }

    pub fn add_node(&mut self, node: &mut Node) {
        node.index = self.0.len() as u32;
        self.0.push(*node);
    }

    pub fn mofify_node(&mut self, node: Node) {
        self.0[node.index as usize] = node
    }

    pub fn get_best_node<const RAPORT: bool>(&self) -> &Node{
        let mut best_node = &self.0[0];
        let mut best_score = f32::MIN;

        for child_index in best_node.children() {
            let child = &self.0[child_index as usize];

            if child.avg_value() > best_score {
                best_score = child.avg_value();
                best_node = &child;
            }
        }

        if RAPORT {
            self.draw_tree_from_root(1);
        }
        best_node
    }

    pub fn draw_tree(&self, node_index: u32, prefix: String, last: bool, is_root: bool, max_depth: i32) {
        if max_depth < 0 {
            return;
        }

        let node = self.get_node(node_index);
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
}

pub struct Search {
    search_tree: SearchTree,
    root_position: Board
}
impl Search {
    pub fn new( board: &Board ) -> Self{
        Self { 
            search_tree: SearchTree::new(),
            root_position: *board
        } 
    }

    fn expand(&mut self, node: &mut Node, current_position: &Board){
        let mut move_list = MoveList::new();
        MoveProvider::generate_moves(&mut move_list, &current_position);

        node.first_child_index = self.search_tree.0.len() as u32;
        node.children_count = move_list.len() as u32;

        for _move in move_list {
            let mut new_node = Node::new(_move);
            new_node.policy_value = 1.0 / node.children_count as f32;
            self.search_tree.add_node(&mut new_node);
        }

        self.search_tree.mofify_node(*node);
    }

    fn select(&self, node: &Node) -> Node{
        let mut best_node = self.search_tree.get_node(node.first_child_index);
        let mut best_puct = f32::MIN;
        let puct_expl = 100.0 * (node.visit_count.max(1) as f32).sqrt();
        for current_child_index in node.children() {
            let child_node = self.search_tree.get_node(current_child_index);
            if child_node.is_terminal {
                continue;
            }

            let current_puct = child_node.avg_value() + (puct_expl * child_node.policy_value / (1 + child_node.visit_count) as f32);

            if current_puct > best_puct {
                best_node = child_node;
                best_puct = current_puct;
            }
        }
        best_node
    }

    fn simulate(&mut self, mut node: Node, current_position: &Board) -> f32 {
        if current_position.is_insufficient_material() {
            node.is_terminal = true;
            self.search_tree.mofify_node(node);
            return 0.5;
        }

        let mut move_list = MoveList::new();
        MoveProvider::generate_moves(&mut move_list, &current_position);

        if move_list.len() == 0 {
            let score = if current_position.is_in_check() { -1.0 } else { 0.5 };
            node.is_terminal = true;
            self.search_tree.mofify_node(node);
            return score;
        }

        1.0 - Evaluation::evaluate(&current_position)
    }

    fn backpropagate(&mut self, selection_history: &mut SelectionHistory, mut result: f32) {
        while let Some(node_index) = selection_history.pop() {
            result = 1.0 - result;

            let mut node = self.search_tree.get_node(node_index);
            node.visit_count += 1;
            node.total_value += result;

            if node.children_count > 0 && node.all_children_terminal(&self.search_tree){
                node.is_terminal = true;
            }

            self.search_tree.0[node.index as usize] = node;
        }
    }

    pub fn run<const UCI_REPORT: bool>(&mut self) -> (Move, f32) {
        let mut root_node = Node::new(Move::NULL);
        self.search_tree.add_node(&mut root_node);

        let root_position = self.root_position;
        self.expand(&mut root_node, &root_position);
        self.search_tree.mofify_node(root_node);

        if root_node.children_count == 0 || root_position.is_insufficient_material() {
            return (Move::NULL, 0.0);
        }

        let mut selection_history = SelectionHistory::new();

        for _ in 0..300000 {
            let mut current_node = self.search_tree.get_node(0);
            let mut current_board = self.root_position;

            selection_history.clear();
            selection_history.push(current_node.index);

            while !current_node.is_leaf() {
                current_node = self.select(&current_node);
                selection_history.push(current_node.index);
                current_board.make_move(current_node._move);
            }

            if current_node.visit_count == 1 {
                self.expand(&mut current_node, &current_board);
                current_node = self.search_tree.get_node(current_node.first_child_index);
                selection_history.push(current_node.index);
                current_board.make_move(current_node._move);
            }

            let current_score = self.simulate(current_node, &current_board);
            self.backpropagate(&mut selection_history, current_score);

            if self.search_tree.get_node(0).is_terminal {
                break;
            }
        }

        let best_node = self.search_tree.get_best_node::<true>();
        (best_node._move, best_node.avg_value())
    }
}