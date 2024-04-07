mod search_tree;
mod node;
mod search_rules;
mod search_params;

pub use search_rules::SearchRules;

use std::time::Instant;
use arrayvec::ArrayVec;
use crate::{core::{Board, Move, MoveList, MoveProvider}, eval::Evaluation};
use self::{node::Node, search_params::SearchParams, search_tree::SearchTree};

type NodeIndex = u32;
type SelectionHistory = ArrayVec<NodeIndex, 128>;

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

    pub fn run(&mut self, search_rules: &SearchRules) -> Move{
        let timer = Instant::now();
        let mut selection_history = SelectionHistory::new();
        let mut search_params = SearchParams::new();
        let root_node = Node::new(Move::NULL);
        self.search_tree.push(&root_node);
        let board = self.root_position;
        self.expand(0, &board);

        while search_rules.continue_search(&search_params)
        {
            selection_history.clear();

            let mut current_node_index = root_node.index;
            let mut current_board = self.root_position;
            selection_history.push(current_node_index);

            let mut depth = 0u32;
            while !self.search_tree[current_node_index].is_leaf() {
                current_node_index = self.select(current_node_index);
                selection_history.push(current_node_index);
                current_board.make_move(self.search_tree[current_node_index]._move);
                depth += 1;
            }

            let node_score = self.simulate(current_node_index, &current_board);

            if !self.search_tree[current_node_index].is_terminal {
                self.expand(current_node_index, &current_board);
            }

            self.backpropagate(&mut selection_history, node_score);

            if search_params.curernt_iterations % 128 == 0 {
                search_params.time_passed = timer.elapsed().as_millis();
            }

            search_params.max_depth = search_params.max_depth.max(depth);
            search_params.total_depth += depth;
            search_params.curernt_iterations += 1;
            search_params.nodes = self.search_tree.node_count();
        }

        self.search_tree.get_best_node()._move
    }

    fn expand(&mut self, node_index: NodeIndex, board: &Board) {
        let mut move_list = MoveList::new();
        MoveProvider::generate_moves(&mut move_list, &board);

        self.search_tree[node_index].first_child_index = self.search_tree.node_count();
        self.search_tree[node_index].children_count = move_list.len() as NodeIndex;

        for _move in move_list {
            let mut new_node = Node::new(_move);
            new_node.index = self.search_tree.node_count();
            new_node.policy_value = 1.0 / self.search_tree[node_index].children_count as f32;
            self.search_tree.push(&new_node);
        }
    }

    fn select(&self, parent_index: NodeIndex) -> NodeIndex {
        let mut best_index = 0;
        let mut best_value = f32::MIN;
        for child_index in self.search_tree[parent_index].children() {
            let current_value = puct(&self.search_tree, parent_index, child_index, 1.41);
            if current_value > best_value {
                best_index = child_index;
                best_value = current_value;
            }
        }
        best_index
    }

    fn simulate(&mut self, node_index: NodeIndex, board: &Board) -> f32 {
        if board.is_insufficient_material() {
            self.search_tree[node_index].is_terminal = true;
            return 0.5;
        }

        let mut move_list = MoveList::new();
        MoveProvider::generate_moves(&mut move_list, &board);

        if move_list.len() == 0 {
            let score = if board.is_in_check() { -1.0 } else { 0.5 };
            self.search_tree[node_index].is_terminal = true;
            return score;
        }

        Evaluation::evaluate(&board)
    }

    fn backpropagate(&mut self, selection_history: &mut SelectionHistory, mut result: f32) {
        while let Some(node_index) = selection_history.pop() {
            result = 1.0 - result;
            self.search_tree[node_index].total_value += result;
            self.search_tree[node_index].visit_count += 1;
        }
    }
}

//PUCT formula V + C * P * (N.max(1).sqrt()/n + 1) where N = number of visits to parent node, n = number of visits to a child
fn puct( search_tree: &SearchTree, parent_index: NodeIndex, child_index: NodeIndex, c: f32 ) -> f32{
    let parent_node = &search_tree[parent_index];
    let child_node = &search_tree[child_index];
    let n = parent_node.visit_count;
    let ni = child_node.visit_count;
    let v = child_node.avg_value();
    let p = child_node.policy_value;

    let numerator = (n.max(1) as f32).sqrt();
    let denominator = ni as f32 + 1.0;
    v + c * p * (numerator/denominator)
}