mod node;
mod phantom_node;
mod qsearch;
mod search_info;
mod search_rules;
mod search_tree;

pub use node::GameResult;
pub use search_info::SearchInfo;
pub use search_rules::SearchRules;
pub use search_tree::SearchTree;
use std::sync::RwLock;

use self::{node::Node, phantom_node::PhantomNode, qsearch::qsearch};
use crate::{
    core::{Board, Move, MoveList, MoveProvider},
    eval::Evaluation,
    search_report::SearchReport,
};
use std::{sync::Arc, time::Instant};

pub struct Search<const LOG: bool> {
    tree: SearchTree,
    interrupt_token: Option<Arc<RwLock<bool>>>,
    search_info: SearchInfo,
}
impl<'a, const LOG: bool> Search<LOG> {
    pub fn new(tree: SearchTree, interrupt_token: Option<Arc<RwLock<bool>>>) -> Self {
        Self { tree, interrupt_token, search_info: SearchInfo::new() }
    }

    pub fn tree(&self) -> &SearchTree {
        &self.tree
    }

    pub fn search_info(&self) -> &SearchInfo {
        &self.search_info
    }

    pub fn reuse_tree(&mut self, board: &Board, previous_board: &Board) {
        if board != previous_board {
            //If positions are not equal we try to find the new position in the tree
            //and reuse the tree. We also reset the search info.
            if self.tree.reuse_tree(board, previous_board) {
                //We want to recalculate policies due to change of root
                //(we flatten policies at root to reduce the chance of
                //missing good move with low policy)
                let root_index = self.tree.root_index();
                self.tree[root_index].recalculate_policies::<true>(board);
            }
        } else if self.tree.node_count() == 0 {
            //If we are using the same tree we want to make sure it has a root
            //(if its a first search there is no previous tree, so root doesn't exist)
            //If that's the case we reset the tree
            self.tree.reset_tree(board);
        }

        self.search_info = SearchInfo::new();
    }

    pub fn run<const PRETTY_PRINT: bool>(&mut self, search_rules: SearchRules, root_position: &Board) -> Move {
        if PRETTY_PRINT && LOG {
            println!("   Depth   Score    Time      Nodes     Speed        Usage   Pv Line");
        }

        let timer = Instant::now();
        let mut current_avg_depth = 0;
        let mut current_max_depth = 0;
        let mut last_report: String = String::new();

        //If tree is complitly empty we want to reset it in order to spawn
        //and expand root node
        if self.tree.node_count() == 0 {
            self.tree.reset_tree(root_position);
        }

        //Iteration loop that breaks, when search rules decide seach should not longer continue
        //or when iteration returns 'true' which is search-break token
        while search_rules.continue_search(&self.search_info, &self.tree) {
            //Initialize and perform one iteration cycle. This cycle covers whole mcts loop
            //including selection, expansion, simulation and backpropagation
            let mut position = *root_position;
            let mut current_depth = 0;
            self.perform_iteration_step(self.tree.root_index(), &mut position, &mut current_depth);

            if self.search_info.current_iterations % 128 == 0 {
                self.search_info.time_passed = timer.elapsed().as_millis();
            }

            //We are upadating all search parameters to prepare it for next iteration or end of the search
            self.search_info.max_depth = self.search_info.max_depth.max(current_depth - 1);
            self.search_info.total_depth += current_depth - 1;
            self.search_info.current_iterations += 1;
            self.search_info.nodes = self.tree.node_count() as u32;

            //If interruption signal was send ('stop' command), we force exit the search
            if let Some(token) = &self.interrupt_token {
                if *token.read().unwrap() {
                    break;
                }
            }

            //when we found forcing line to end the gmae, we end the search
            if self.tree[0].is_terminal() {
                break;
            }

            //Draws the search report, when average selection depth or max selection depth improved,
            //we provide last raport to make sure we don't print duplicates
            if self.search_info.get_avg_depth() > current_avg_depth
                || (self.search_info.get_avg_depth() != current_avg_depth
                    && self.search_info.max_depth > current_max_depth)
            {
                self.search_info.time_passed = timer.elapsed().as_millis();
                if LOG {
                    self.print_report::<PRETTY_PRINT>(self.search_info, &mut last_report);
                }
                current_avg_depth = current_avg_depth.max(self.search_info.get_avg_depth());
                current_max_depth = current_max_depth.max(self.search_info.max_depth);
            }
        }

        //We want to print final search report, we provide
        //last raport to make sure we don't print duplicates
        self.search_info.time_passed = timer.elapsed().as_millis();
        if LOG {
            self.print_report::<PRETTY_PRINT>(self.search_info, &mut last_report);
        }

        self.tree.get_best_phantom().mv()
    }

    fn perform_iteration_step(
        &mut self,
        current_node_index: i32,
        current_board: &mut Board,
        current_depth: &mut u32,
    ) -> f32 {
        *current_depth += 1;

        if current_node_index == -1 {
            println!("WTF");
            println!("DEPTH {current_depth}");
            println!("ROOT {}", self.tree.root_index());
        }
        self.tree.make_recently_used(current_node_index);

        //Data to trace phantom parent of currently processed node
        let parent_index = self.tree[current_node_index].parent();
        let child_index = self.tree[current_node_index].child();

        let mut child_result = GameResult::None;
        let parent_visits = self.tree.get_phantom(parent_index, child_index).visits();

        //If node is terminal we don't need to look fuether. We can just return the value of terminal state
        //of this node. If node had no visits (leaf node), then we simulate the node and return it's value. We will
        //expand this node on second visit
        let mut score = if self.tree[current_node_index].is_terminal() || parent_visits == 0 {
            self.get_node_score(current_node_index, &current_board)
        } else {
            //On second visit we extend the node, if it wasn't already extended.
            //This allows us to reduce amount of time we evaluate policy net
            if !self.tree[current_node_index].is_extended() {
                self.tree[current_node_index].expand::<false>(&current_board);
            }

            //Select best phantom child (selection returns index of the move from it's parent)
            //based on PUCT formula
            let new_child_index = self.select_node(current_node_index);

            //Index being equal to MAX means that selected child is terminal node and
            //when that happens we want to return it's terminal value, otherwise we
            //process selected child and move deeper into the tree, until we find a leaf node
            //or terminal state
            if new_child_index == usize::MAX {
                self.get_node_score(current_node_index, &current_board)
            } else {
                //Extract phantom of selected child and save index of corresponding tree node
                let selected_node_phantom = self.tree.get_phantom(current_node_index, new_child_index);
                let mut child_node_index = selected_node_phantom.index();

                current_board.make_move(selected_node_phantom.mv());

                //If index of corresponding tree node is equal to -1, it means that node
                //doesn't exist on a tree, and we have to create it
                if child_node_index == -1 {
                    //Create new node, assaign it's default values and it's game result and add it to the tree
                    let selected_node_result = self.get_node_result(&current_board);
                    child_node_index =
                        self.tree.push(Node::new(selected_node_result, current_node_index, new_child_index));
                    self.tree.get_phantom_mut(current_node_index, new_child_index).set_index(child_node_index);
                }

                //Save result of processed node for backpropagation stage and
                //perform another iteration step deeper into the tree
                child_result = self.tree[child_node_index].result();
                self.perform_iteration_step(child_node_index, current_board, current_depth)
            }
        };

        //Inverse the score to adapt to side to move perspective.
        //MCTS always selects highest score move, and our opponents wants
        //to select worst move for us, so we have to alternate score as we
        //backpropagate it up the tree
        score = 1.0 - score;

        //Updates currently processed phantom node. Separation of phantom node and actual node,
        //allows for easier implementation of MCGS and replacing old nodes with new ones, when tree
        //is full
        self.tree.get_phantom_mut(parent_index, child_index).apply_score(score);

        //If this node lost then we can backpropagate win one step up, because we can assume
        //that our opponent will select mate as their move
        if let GameResult::Lose(n) = child_result {
            self.tree[current_node_index].set_result(GameResult::Win(n + 1));
        }

        self.tree.make_recently_used(current_node_index);

        score
    }

    fn select_node(&mut self, current_node_index: i32) -> usize {
        if self.tree[current_node_index].children().len() == 0 {
            panic!("trying to pick from no children!");
        }

        //Initialize all variables about currently processed node
        let node = &self.tree[current_node_index];
        let parent = node.parent();
        let action = node.child();
        let parent_phantom = self.tree.get_phantom(parent, action);

        let mut proven_loss = true;
        let mut win_len = 0;
        let mut best = 0;
        let mut max = f32::NEG_INFINITY;
        let c = 1.41;

        //Iterate though all children of the node and calculate puct value of each of them in
        //order to find the child with the highest PUCT score
        for (i, child_phantom) in node.children().iter().enumerate() {
            //If node has not been visited yet then we don't yet know if it is terminal node or not
            let puct = if child_phantom.visits() == 0 {
                proven_loss = false;
                puct(parent_phantom, child_phantom, c)
            } else {
                //If node has been spawned, then we can extract it from the tree and check
                //if result of this node is winning. If node hasn't been spawned yet, then we
                //still don't know, if it is terminal node or not
                if child_phantom.index() != -1 {
                    let child_node = &self.tree[child_phantom.index()];

                    if let GameResult::Win(n) = child_node.result() {
                        win_len = n.max(win_len);
                    } else {
                        proven_loss = false;
                    }
                } else {
                    proven_loss = false;
                }

                puct(parent_phantom, child_phantom, c)
            };

            if puct > max {
                max = puct;
                best = i;
            }
        }

        //If all children are winning, then it's force lose for the other side, so we can
        //backpropagate lose one step up the tree
        if proven_loss {
            self.tree[current_node_index].set_result(GameResult::Lose(win_len + 1));
            return usize::MAX;
        }

        best
    }

    fn get_node_score(&self, node_index: i32, board: &Board) -> f32 {
        match self.tree[node_index].result() {
            GameResult::None => sigmoid(qsearch(board, -30_000, 30_000, 0)),
            GameResult::Win(_) => 1.0,
            GameResult::Lose(_) => 0.0,
            GameResult::Draw => 0.5,
        }
    }

    fn get_node_result(&self, board: &Board) -> GameResult {
        if board.is_insufficient_material() || board.three_fold() || board.half_moves >= 100 {
            return GameResult::Draw;
        }

        let mut move_list = MoveList::new();
        MoveProvider::generate_moves::<false>(&mut move_list, &board);

        if move_list.len() == 0 {
            return if board.is_in_check() { GameResult::Lose(0) } else { GameResult::Draw };
        }

        GameResult::None
    }

    fn print_report<const PRETTY_PRINT: bool>(&mut self, search_info: SearchInfo, last_report: &mut String) {
        let best_phantom = self.tree.get_best_phantom();
        let game_result =
            if best_phantom.index() != -1 { self.tree[best_phantom.index()].result() } else { GameResult::None };
        let report = SearchReport::print_report::<PRETTY_PRINT>(
            &search_info,
            self.tree.get_pv_line(),
            best_phantom.avg_score(),
            game_result,
            &self.tree,
        );

        if report != *last_report {
            println!("{report}");
        }

        *last_report = report
    }
}

//PUCT formula V + C * P * (N.max(1).sqrt()/n + 1) where N = number of visits to parent node, n = number of visits to a child
fn puct(parent: &PhantomNode, child: &PhantomNode, c: f32) -> f32 {
    let n = parent.visits();
    let ni = child.visits();
    let v = child.avg_score();
    let p = child.policy();

    let numerator = (n.max(1) as f32).sqrt();
    let denominator = ni as f32 + 1.0;
    v + c * p * (numerator / denominator)
}

fn sigmoid(input: i32) -> f32 {
    1.0 / (1.0 + (-input as f32 / 400.0).exp())
}
