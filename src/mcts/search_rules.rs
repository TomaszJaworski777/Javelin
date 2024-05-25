use crate::{mcts::SearchTree, options::Options};

use super::search_info::SearchInfo;

#[derive(Clone, Copy)]
pub struct SearchRules {
    pub time_for_move: u64,
    pub max_depth: u32,
    pub max_nodes: u32,
    pub infinite: bool,
}
impl SearchRules {
    pub fn new() -> Self {
        Self { time_for_move: 0, max_depth: 0, max_nodes: 0, infinite: false }
    }

    pub fn continue_search(&self, search_info: &SearchInfo, tree: &SearchTree) -> bool {
        if tree.node_count() + 218 >= tree.capacity() && false {
            return false;
        }

        if self.infinite {
            return true;
        }

        if self.max_nodes > 0 && (search_info.current_iterations - search_info.previous_iterations) >= self.max_nodes as i32 {
            return false;
        }

        if self.max_depth > 0 && (search_info.get_avg_depth() - search_info.start_avg_depth) >= self.max_depth {
            return false;
        }

        if self.time_for_move > 0 && search_info.time_passed >= self.time_for_move as u128 {
            return false;
        }

        true
    }

    pub fn calculate_time(time_remaining: u64, time_increment: u64, moves_to_go: u64) -> u64 {
        let divider = if moves_to_go > 0 { moves_to_go } else { 20 };
        (time_remaining / divider.max(1) + time_increment / 2 - Options::get("MoveOverhead").get_value::<u64>())
            .max(1)
            .min(time_remaining)
    }
}
