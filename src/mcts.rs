mod search_tree;
mod node;

use arrayvec::ArrayVec;
use crate::{board::Board, core_structs::{Move, MoveList}, eval::Evaluation, movegen::MoveProvider};

use self::{node::Node, search_tree::SearchTree};

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

    pub fn run(&mut self) -> Move{
        //Initialize the search
        let mut selection_history = SelectionHistory::new();
        //add root node to the tree
        let root_node = Node::new(Move::NULL);
        self.search_tree.push(&root_node);
        //expand root node
        let board = self.root_position;
        self.expand(0, &board);
        

        //iterate
        for _ in 0..300000
        {
            selection_history.clear();

            //prepare current_node and current_board
            let mut current_node_index = root_node.index;
            let mut current_board = self.root_position;
            selection_history.push(current_node_index);

            //select new node based on the best uct
            while !self.search_tree[current_node_index].is_leaf() {
                current_node_index = self.select(current_node_index);
                selection_history.push(current_node_index);
                current_board.make_move(self.search_tree[current_node_index]._move);
            }
            //simulate the node
            let node_score = self.simulate(current_node_index, &current_board);

            //expand the node but dont simulate any new children
            if !self.search_tree[current_node_index].is_terminal {
                self.expand(current_node_index, &current_board);
            }

            //backpropagate the value from the node up the tree
            self.backpropagate(&mut selection_history, node_score);
        }

        //get best move based on avg_value
        self.search_tree.draw_tree_from_root(1);
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
            self.search_tree.push(&new_node);
        }
    }

    fn select(&self, parent_index: NodeIndex) -> NodeIndex {
        let mut best_index = 0;
        let mut best_value = f32::MIN;
        for child_index in self.search_tree[parent_index].children() {
            let current_value = uct(&self.search_tree, parent_index, child_index, 77.7);
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

//UCT formula V + C * (N.max(1).ln()/n + 0.0000001).sqrt() where N = number of visits to parent node, n = number of visits to a child
//later replace with puct --> V + C * P * (N.max(1).sqrt()/n + 1) where N = number of visits to parent node, n = number of visits to a child
fn uct( search_tree: &SearchTree, parent_index: NodeIndex, child_index: NodeIndex, c: f32 ) -> f32{
    let parent_node = &search_tree[parent_index];
    let child_node = &search_tree[child_index];
    let n = parent_node.visit_count;
    let ni = child_node.visit_count;
    let v = child_node.avg_value();

    let numerator = (n.max(1) as f32).ln();
    let denominator = (ni as f32).max(0.0000001);
    v + c * (numerator/denominator).sqrt()
}