mod node;
mod search_params;
mod search_rules;
mod search_tree;
mod qsearch;

pub use search_params::SearchParams;
pub use search_rules::SearchRules;
pub use search_tree::SearchTree;

use self::{node::Node, qsearch::qsearch};
use crate::{
    core::{Board, Move, MoveList, MoveProvider},
    uci::Uci,
};
use arrayvec::ArrayVec;
use std::{sync::mpsc::Receiver, time::Instant};

type NodeIndex = u32;
type SelectionHistory = ArrayVec<NodeIndex, 128>;

pub struct Search<'a> {
    search_tree: SearchTree,
    root_position: Board,
    interruption_channel: &'a Receiver<()>,
}
impl<'a> Search<'a> {
    pub fn new(board: &Board, interruption_channel: &'a Receiver<()>) -> Self {
        Self { search_tree: SearchTree::new(), root_position: *board, interruption_channel }
    }

    pub fn run(&mut self, search_rules: &SearchRules) -> (Move, &SearchTree) {
        let timer = Instant::now();
        let mut selection_history = SelectionHistory::new();
        let mut search_params = SearchParams::new();
        let root_node = Node::new(Move::NULL);
        self.search_tree.push(&root_node);
        let board = self.root_position;
        self.expand(0, &board);

        let mut current_avg_depth = 0;
        while search_rules.continue_search(&search_params) && search_params.nodes + 256 < u32::MAX {
            selection_history.clear();

            let mut current_node_index = root_node.index;
            let mut current_board = self.root_position;
            selection_history.push(current_node_index);

            let mut depth = 0u32;
            while !self.search_tree[current_node_index].is_leaf() {
                current_node_index = self.select(current_node_index);
                selection_history.push(current_node_index);
                current_board.make_move(self.search_tree[current_node_index].mv);
                depth += 1;
            }

            let node_score = self.simulate(current_node_index, &current_board);

            if !self.search_tree[current_node_index].is_terminal {
                self.expand(current_node_index, &current_board);
            }

            {
                self.backpropagate(&mut selection_history, node_score);

                if search_params.curernt_iterations % 128 == 0 {
                    search_params.time_passed = timer.elapsed().as_millis();
                }
    
                search_params.max_depth = search_params.max_depth.max(depth);
                search_params.total_depth += depth;
                search_params.curernt_iterations += 1;
                search_params.nodes = self.search_tree.node_count();
    
                if let Ok(_) = self.interruption_channel.try_recv() {
                    search_params.time_passed = timer.elapsed().as_millis();
                    Uci::print_raport(&search_params, self.search_tree.get_pv_line());
                    break;
                }
    
                if search_params.get_avg_depth() > current_avg_depth {
                    search_params.time_passed = timer.elapsed().as_millis();
                    Uci::print_raport(&search_params, self.search_tree.get_pv_line());
                    current_avg_depth = search_params.get_avg_depth();
                }
            }
        }

        (self.search_tree.get_best_node().mv, &self.search_tree)
    }

    fn expand(&mut self, node_index: NodeIndex, board: &Board) {
        let mut move_list = MoveList::new(); 
        MoveProvider::generate_moves::<false>(&mut move_list, &board);

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
        if board.is_insufficient_material() || board.three_fold() || board.half_moves >= 100 {
            self.search_tree[node_index].is_terminal = true;
            return 0.5;
        }
    
        let mut move_list = MoveList::new();
        MoveProvider::generate_moves::<false>(&mut move_list, &board);
    
        if move_list.len() == 0 {
            let score = if board.is_in_check() { -1.0 } else { 0.5 };
            self.search_tree[node_index].is_terminal = true;
            return score;
        }

        sigmoid(qsearch(&board, -30000, 30000))
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
fn puct(search_tree: &SearchTree, parent_index: NodeIndex, child_index: NodeIndex, c: f32) -> f32 {
    let parent_node = &search_tree[parent_index];
    let child_node = &search_tree[child_index];
    let n = parent_node.visit_count;
    let ni = child_node.visit_count;
    let v = child_node.avg_value();
    let p = child_node.policy_value;

    let numerator = (n.max(1) as f32).sqrt();
    let denominator = ni as f32 + 1.0;
    v + c * p * (numerator / denominator)
}

fn sigmoid(input: i32) -> f32 {
    1.0 / (1.0 + (-input as f32 / 400.0).exp())
}