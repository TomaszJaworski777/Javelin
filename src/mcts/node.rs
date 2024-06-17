use crate::{
    core::{Board, MoveList, MoveProvider},
    mcts::Evaluation,
    options::Options,
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
    backward_link: i32,
}
impl Node {
    pub fn new(result: GameResult, parent: i32, child: usize) -> Self {
        Self { children: Vec::new(), result, parent, child: child as u16, forward_link: -1, backward_link: -1 }
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

    pub fn backward_link(&self) -> i32 {
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

    pub fn clear_parent(&mut self) {
        self.parent = -1;
        self.child = 0;
    }

    pub fn expand<const ROOT: bool>(&mut self, board: &Board) {
        //Generate all possible moves from the node
        let mut move_list = MoveList::new();
        MoveProvider::generate_moves::<false>(&mut move_list, &board);

        let is_single_move = move_list.len() == 1;
        let mut max_policy_value = f32::NEG_INFINITY;

        //Generate inputs for the policy network
        let policy_inputs = Evaluation::get_policy_inputs(board);
        self.children = Vec::with_capacity(move_list.len());

        //Prebake new children with raw policy
        for mv in move_list {
            //If there is only one move, policy is not needed
            let policy = if is_single_move { 1.0 } else { Evaluation::get_policy_value(board, &mv, &policy_inputs) };
            self.children.push(PhantomNode::new((policy * 1000.0) as i32, mv, 0.0));

            //Save highest policy for later softmax
            max_policy_value = max_policy_value.max(policy);
        }

        let mut total_policy = 0.0;
        let root_pst = Options::root_pst();

        //Iterate through created children to apply first part of softmax and pst dampening
        for child_phantom in self.children_mut() {
            let policy: f32 = child_phantom.index() as f32 / 1000.0;

            let policy = if ROOT {
                ((policy - max_policy_value) / root_pst).exp()
            } else {
                (policy - max_policy_value).exp()
            };

            child_phantom.set_index((policy * 1000.0) as i32);

            total_policy += policy;
        }

        //Iterate again to apply second part of softmax
        for child_phantom in self.children_mut() {
            let policy_value = child_phantom.index() as f32 / 1000.0;
            let policy = policy_value / total_policy;
            child_phantom.update_policy(policy);
            child_phantom.set_index(-1);
        }
    }

    pub fn recalculate_policies<const ROOT: bool>(&mut self, board: &Board) {
        let is_single_move = self.children().len() == 1;
        let mut max_policy_value = f32::NEG_INFINITY;

        //Generate inputs for the policy network
        let policy_inputs = Evaluation::get_policy_inputs(board);

        //Update children
        for child_phantom in self.children_mut() {
            //If there is only one move, policy is not needed
            let policy = if is_single_move {
                1.0
            } else {
                Evaluation::get_policy_value(board, &child_phantom.mv(), &policy_inputs)
            };
            child_phantom.update_policy(policy);

            //Save highest policy for later softmax
            max_policy_value = max_policy_value.max(policy);
        }

        let mut total_policy = 0.0;
        let root_pst = Options::root_pst();

        //Iterate through created children to apply first part of softmax and pst dampening
        for child_phantom in self.children_mut() {
            let mut policy: f32 = child_phantom.policy();

            policy = if ROOT {
                ((policy - max_policy_value) / root_pst).exp()
            } else {
                (policy - max_policy_value).exp()
            };

            child_phantom.update_policy(policy);

            total_policy += policy;
        }

        //Iterate again to apply second part of softmax
        for child_phantom in self.children_mut() {
            let policy = child_phantom.policy() / total_policy;
            child_phantom.update_policy(policy);
        }
    }
}
