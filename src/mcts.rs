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

        if RAPORT {
            print!("Selection raport:\n");
        }

        for child_index in best_node.children() {
            let child = &self.0[child_index as usize];

            if RAPORT {
                print!("    {} Q({:2}%) N({}) P({}) C({})\n", child._move.to_string(), child.avg_value() * 100.0, child.visit_count, child.policy_value, child.children_count);
            }

            if child.avg_value() > best_score {
                best_score = child.avg_value();
                best_node = &child;
            }
        }
        best_node
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
        node.policy_value = 1.0 / node.children_count as f32;

        for _move in move_list {
            let mut new_node = Node::new(_move);
            new_node.policy_value = 0.25;
            self.search_tree.add_node(&mut new_node);
        }

        self.search_tree.mofify_node(*node);
    }

    fn select(&self, node: &Node) -> Node{
        let mut best_node = self.search_tree.get_node(node.first_child_index);
        let mut best_puct = f32::MIN;
        let root_node_visits = self.search_tree.get_node(0).visit_count;
        for current_child_index in node.children() {
            let child_node = self.search_tree.get_node(current_child_index);
            if child_node.is_terminal {
                continue;
            }

            let current_puct = Search::puct(root_node_visits, child_node, 100.0);

            if current_puct > best_puct {
                best_node = child_node;
                best_puct = current_puct;
            }
        }
        best_node
    }

    fn puct(root_visits: u32, node: Node, c_value: f32) -> f32 {
        node.avg_value() + node.policy_value * c_value * ((root_visits as f32 + 1.000001).ln()/(node.visit_count as f32 + 0.000001)).sqrt()
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

        Evaluation::evaluate(&current_position)
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