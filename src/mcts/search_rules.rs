use super::search_params::SearchParams;

pub struct SearchRules{
    pub time_for_move: u64,
    pub max_depth: u32,
    pub max_iterations: u32,
    pub max_nodes: u32,
    pub infinite: bool,
}
impl SearchRules {
    pub fn new() -> Self{
        Self{
            time_for_move: 0,
            max_depth: 0,
            max_iterations: 0,
            max_nodes: 0,
            infinite: false
        }
    }

    pub fn continue_search(&self, search_params: &SearchParams) -> bool {
        if self.infinite {
            return true;
        }

        if self.max_iterations > 0 && search_params.curernt_iterations >= self.max_iterations {
            return false;
        }

        if self.max_depth > 0 && search_params.get_avg_depth() >= self.max_depth {
            return false;
        }

        if self.max_nodes > 0 && search_params.nodes >= self.max_nodes {
            return false;
        }

        if self.time_for_move > 0 && search_params.time_passed >= self.time_for_move as u128{
            return false;
        }

        true
    }

    pub fn calculate_time(time_remaining: u64, time_increment: u64, moves_to_go: u64) -> u64{
        let divider = if moves_to_go > 0 { moves_to_go } else { 20 };
        (time_remaining / divider.max(1) + time_increment / 2).max(10)
    }
}