mod node;
mod qsearch;
mod search_params;
mod search_rules;
mod search_tree;

pub use node::GameResult;
pub use search_params::SearchParams;
pub use search_rules::SearchRules;
pub use search_tree::SearchTree;

use self::{node::Node, qsearch::qsearch};
use crate::{
    core::{Board, Move, MoveList, MoveProvider, Side},
    eval::Evaluation,
    search_raport::SearchRaport,
};
use std::{sync::mpsc::Receiver, time::Instant};

type NodeIndex = u32;
type SelectionHistory = Vec<NodeIndex>;

pub struct Search<'a, const LOG: bool> {
    search_tree: SearchTree,
    root_position: Board,
    interruption_channel: Option<&'a Receiver<()>>,
    timer: Instant,
    search_params: SearchParams,
    search_rules: SearchRules,
    selection_history: SelectionHistory,
    root_node: Node,
    current_avg_depth: u32,
}
impl<'a, const LOG: bool> Search<'a, LOG> {
    pub fn new(board: &Board, interruption_channel: Option<&'a Receiver<()>>, search_rules: SearchRules) -> Self {
        let mut search = Self {
            search_tree: SearchTree::new(),
            root_position: *board,
            interruption_channel,
            timer: Instant::now(),
            search_params: SearchParams::new(),
            search_rules,
            selection_history: SelectionHistory::new(),
            root_node: Node::new(Move::NULL),
            current_avg_depth: 0,
        };

        //Initialize root node, add it to a tree and expand it
        let root_node = Node::new(Move::NULL);
        search.search_tree.push(&root_node);
        let board = search.root_position;
        search.expand_node(0, &board);
        search.root_node = root_node;

        search
    }

    pub fn run<const PRETTY_PRINT: bool>(&mut self) -> (Move, &SearchTree, SearchParams) {
        if PRETTY_PRINT && LOG {
            println!("   Depth   Score    Time      Nodes     Speed        Usage   Pv Line");
        }

        //Prepare all variables needed for next iteration, increment current iteration counter
        self.init_next_iteration::<PRETTY_PRINT>()
    }

    fn init_next_iteration<const PRETTY_PRINT: bool>(&mut self) -> (Move, &SearchTree, SearchParams) {
        self.selection_history.clear();
        self.selection_history.push(0);
        self.search_params.curernt_iterations += 1;

        //Start selection loop for root node and root position
        let mut board = self.root_position;
        self.selection_step::<PRETTY_PRINT>(0, &mut board, 0)
    }

    fn selection_step<const PRETTY_PRINT: bool>(
        &mut self,
        mut current_node_index: u32,
        current_board: &mut Board,
        mut depth: u32,
    ) -> (Move, &SearchTree, SearchParams) {
        //We select best child of a node. If selected child is checkmate
        //or if it's root. Then we can end the search, due to all children
        //of root being terminal nodes. TODO: what if the node is in the stalemate?
        //we probb should not visit it anymore
        current_node_index = self.select_best_child(current_node_index);

        if let GameResult::Win(_) = self.search_tree[current_node_index].result {
            return self.end_step::<PRETTY_PRINT>(true);
        }

        if current_node_index == 0 {
            return self.end_step::<PRETTY_PRINT>(true);
        }

        //We apply the selected move, and increase selection depth
        self.selection_history.push(current_node_index);
        current_board.make_move(self.search_tree[current_node_index].mv);
        depth += 1;

        //If selected node is a leaf, we move to expansion step.
        //Otherwise we want to select further
        if self.search_tree[current_node_index].is_leaf() {
            return self.expand_step::<PRETTY_PRINT>(current_node_index, current_board, depth);
        }

        self.selection_step::<PRETTY_PRINT>(current_node_index, current_board, depth)
    }

    fn expand_step<const PRETTY_PRINT: bool>(
        &mut self,
        current_node_index: u32,
        current_board: &mut Board,
        depth: u32,
    ) -> (Move, &SearchTree, SearchParams) {
        //If node has beem visited before, we want to expand it further
        //and select one of its children as current node for simulation
        //If no, then we want to simulate it first
        if !self.search_tree[current_node_index].is_terminal() && self.search_tree[current_node_index].visit_count > 0 {
            self.expand_node(current_node_index, current_board);
            return self.selection_step::<PRETTY_PRINT>(current_node_index, current_board, depth);
        }

        self.simulation_step::<PRETTY_PRINT>(current_node_index, current_board, depth)
    }

    fn simulation_step<const PRETTY_PRINT: bool>(
        &mut self,
        current_node_index: u32,
        current_board: &Board,
        depth: u32,
    ) -> (Move, &SearchTree, SearchParams) {
        //We simulate currently selected node and then backpropage the result
        //down the tree
        let node_score = self.simulate_node(current_node_index, &current_board);
        self.backpropagation_step::<PRETTY_PRINT>(depth, node_score)
    }

    fn backpropagation_step<const PRETTY_PRINT: bool>(
        &mut self,
        depth: u32,
        score: f32,
    ) -> (Move, &SearchTree, SearchParams) {
        self.backpropagate_score(score);
        return self.end_iteration::<PRETTY_PRINT>(depth);
    }

    fn end_iteration<const PRETTY_PRINT: bool>(&mut self, depth: u32) -> (Move, &SearchTree, SearchParams) {
        //We update time only every 128 iterations to reduce workload during search
        if self.search_params.curernt_iterations % 128 == 0 {
            self.search_params.time_passed = self.timer.elapsed().as_millis();
        }

        //We are upadating all search parameters to prepare it for next iteration or end of the search
        self.search_params.max_depth = self.search_params.max_depth.max(depth);
        self.search_params.total_depth += depth;
        self.search_params.curernt_iterations += 1;
        self.search_params.nodes = self.search_tree.node_count();

        //If interruption signal was send ('stop' command), we force exit the search
        if let Some(reciver) = self.interruption_channel {
            if let Ok(_) = reciver.try_recv() {
                return self.end_step::<PRETTY_PRINT>(true);
            }
        }

        //Draws the search report, when average selection depth improved
        if self.search_params.get_avg_depth() > self.current_avg_depth {
            self.search_params.time_passed = self.timer.elapsed().as_millis();
            let best_node = self.search_tree.get_best_node();
            if LOG {
                SearchRaport::print_raport::<PRETTY_PRINT>(
                    &self.search_params,
                    self.search_tree.get_pv_line(),
                    best_node.avg_value(),
                    best_node.result,
                );
            }
            self.current_avg_depth = self.search_params.get_avg_depth();
        }

        //Decided based on search rules if search shuold be continued or interrupted
        if self.search_rules.continue_search(&self.search_params) {
            return self.init_next_iteration::<PRETTY_PRINT>();
        }

        //We don't want to print search report if search ended due to reaching max depth
        self.end_step::<PRETTY_PRINT>(
            self.search_rules.max_depth == 0 || self.search_params.get_avg_depth() < self.search_rules.max_depth,
        )
    }

    fn end_step<const PRETTY_PRINT: bool>(&mut self, show_report: bool) -> (Move, &SearchTree, SearchParams) {
        self.search_params.time_passed = self.timer.elapsed().as_millis();
        let best_node = self.search_tree.get_best_node();

        if LOG && show_report {
            SearchRaport::print_raport::<PRETTY_PRINT>(
                &self.search_params,
                self.search_tree.get_pv_line(),
                best_node.avg_value(),
                best_node.result,
            );
        }

        (self.search_tree.get_best_node().mv, &self.search_tree, self.search_params)
    }

    fn expand_node(&mut self, node_index: NodeIndex, board: &Board) {
        //Generate all possible moves from the node
        let mut move_list = MoveList::new();
        MoveProvider::generate_moves::<false>(&mut move_list, &board);

        //Save children data to the parent node
        self.search_tree[node_index].first_child_index = self.search_tree.node_count();
        self.search_tree[node_index].children_count = move_list.len() as NodeIndex;

        //Get policy values from the policy network
        let policy_values = Evaluation::get_policy_values(&board, &move_list);

        for mv in move_list {
            //Create a new node and initialize it's default values
            let mut new_node = Node::new(mv);
            new_node.index = self.search_tree.node_count();

            //Calculate policy index -> piece_type * 64 + target_square
            //We flip the board for neural network to always present it from side to move POV
            //So we also need to flip the target_square of the move
            let base_index = (board.get_piece_on_square(mv.get_from_square()).0 - 1) * 64;
            let index = base_index
                + if board.side_to_move == Side::WHITE {
                    mv.get_to_square().get_value()
                } else {
                    mv.get_to_square().get_value() ^ 56
                };

            //Apply policy and push the node to the tree
            new_node.policy_value = policy_values[index];
            self.search_tree.push(&new_node);
        }
    }

    fn select_best_child(&self, parent_index: NodeIndex) -> NodeIndex {
        //We select the best child node, based on the PUCT formula
        let mut best_index = 0;
        let mut best_value = f32::MIN;
        for child_index in self.search_tree[parent_index].children() {
            //We don't want to select terminal node
            //If all children are terminal we will return root index
            if self.search_tree[child_index].is_terminal() {
                if matches!(self.search_tree[child_index].result, GameResult::Win(_))
                    && self.search_tree[parent_index].index == 0
                {
                    return child_index;
                }
                continue;
            }

            //Calculate current PUCT based on the formula
            let current_value = puct(&self.search_tree, parent_index, child_index, 1.41);

            if current_value > best_value {
                best_index = child_index;
                best_value = current_value;
            }
        }
        best_index
    }

    fn simulate_node(&mut self, node_index: NodeIndex, board: &Board) -> f32 {
        //we are checking all draw conditions and return draw state if it occurs
        if board.is_insufficient_material() || board.three_fold() || board.half_moves >= 100 {
            self.search_tree[node_index].result = GameResult::Draw;
            return 0.5;
        }

        //We are generating move list in order to detect stalemates and checkmates
        let mut move_list = MoveList::new();
        MoveProvider::generate_moves::<false>(&mut move_list, &board);

        if move_list.len() == 0 {
            let score = if board.is_in_check() { 0.0 } else { 0.5 };
            if score == 0.0 {
                self.search_tree[node_index].result = GameResult::Win(0);
            } else {
                self.search_tree[node_index].result = GameResult::Draw;
            }
            return score;
        }

        //Finally if position is not a draw, we return the sigmoid of qsearch result based on
        //value neural network
        sigmoid(qsearch(&board, -30000, 30000, 0))
    }

    fn backpropagate_score(&mut self, mut result: f32) {
        //We are iterating through selectiong history and selecting nodes one by one.
        //Then we are applying alternating score to them (score = 1 - score), in order to
        //help with evaluating the position from engine perspective. Mcts always wants to pick
        //best node, so we are inverting the score for oppotent to make sure that it picks best move for
        //the opponent
        let mut previous_node_index = 0;
        while let Some(node_index) = self.selection_history.pop() {
            result = 1.0 - result;

            //This occurs when node that we are currently modifying is a leaf-node
            //For this node we don't want to do any operations that depend on state of it's children
            if previous_node_index == 0 {
                self.search_tree[node_index].total_value += result;
                self.search_tree[node_index].visit_count += 1;
                previous_node_index = node_index;
                continue;
            }

            //Here we are backpropagating the game result.

            //If any node is winning for player, we can assume that it will be chosen,
            //therefore we can backpropgage it to it's parent
            if let GameResult::Win(n) = self.search_tree[previous_node_index].result {
                self.search_tree[node_index].result = GameResult::Lose(n + 1);
                self.search_tree[node_index].visit_count += 1;
                self.search_tree[node_index].total_value = 0.0;

            //If all children are losing then this node has to be winning for the other player,
            //because no matter what player will chose, it is always lost
            } else if let Some(n) = self.search_tree[node_index].all_children_lost(&self.search_tree) {
                self.search_tree[node_index].result = GameResult::Win(n + 1);
                self.search_tree[node_index].visit_count += 1;
                self.search_tree[node_index].total_value = self.search_tree[node_index].visit_count as f32;

            //When all children result in draw then we can backpropgate draw down the tree,
            //due to it's being forced
            } else if self.search_tree[node_index].all_children_draw(&self.search_tree) {
                self.search_tree[node_index].result = GameResult::Draw;
                self.search_tree[node_index].visit_count += 1;
                self.search_tree[node_index].total_value = (self.search_tree[node_index].visit_count as f32) / 2.0;
            } else {
                self.search_tree[node_index].total_value += result;
                self.search_tree[node_index].visit_count += 1;
            }

            previous_node_index = node_index;
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
