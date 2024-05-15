use crate::{
    core::{Board, MoveList, MoveProvider, Side},
    mcts::Evaluation,
};

use super::phantom_node::PhantomNode;

#[derive(Clone, Copy, PartialEq)]
pub enum GameResult {
    None,
    Lose(u8),
    Draw,
    Win(u8),
}

#[derive(Clone)]
pub struct Node {
    children: Vec<PhantomNode>,
    result: GameResult,
    parent: i32,
    child: u16,
    forward_link: i32,
    backward_link: i32
}
impl Node {
    pub fn new(result: GameResult, parent: i32, child: usize) -> Self {
        Self { 
            children: Vec::new(), 
            result, 
            parent, 
            child: child as u16,
            forward_link: -1,
            backward_link: -1
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.result != GameResult::None
    }

    pub fn is_extended(&self) -> bool {
        self.is_terminal() || self.children.len() > 0
    }

    pub fn children(&self) -> &[PhantomNode] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut [PhantomNode] {
        &mut self.children
    }

    pub fn result(&self) -> GameResult {
        self.result
    }

    pub fn set_result(&mut self, result: GameResult) {
        self.result = result
    }

    pub fn parent(&self) -> i32 {
        self.parent
    }

    pub fn child(&self) -> usize {
        self.child as usize
    }

    pub fn forward_link(&self) -> i32 {
        self.forward_link
    }

    pub fn set_forward_link(&mut self, new_value: i32) {
        self.forward_link = new_value
    }

    pub fn backward_link_link(&self) -> i32 {
        self.backward_link
    }

    pub fn set_backward_link(&mut self, new_value: i32) {
        self.backward_link = new_value
    }

    pub fn clear(&mut self) {
        self.children.clear();
        self.result = GameResult::None;
        self.forward_link = -1;
        self.backward_link = -1;
    }

    pub fn expand(&mut self, board: &Board) {
        //Generate all possible moves from the node
        let mut move_list = MoveList::new();
        MoveProvider::generate_moves::<false>(&mut move_list, &board);

        //Get policy values from the policy network
        let policy_values = Evaluation::get_policy_values(&board, &move_list);

        for mv in move_list {
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

            self.children.push(PhantomNode::new(-1, mv, policy_values[index]));
        }
    }
}
