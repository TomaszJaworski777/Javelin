use super::search_params::SearchParams;

pub struct SearchRules{
    pub time_for_move: f64,
    pub max_depth: u32,
    pub max_iterations: u32,
    pub infinite: bool,
}
impl SearchRules {
    pub fn continue_search(&self, search_params: &SearchParams) -> bool {
        

        true
    }

    pub fn calculate_time(time_remaining: f64, time_increment: f32, moves_to_go: u32) -> f64{
        let divider = if moves_to_go > 0 { moves_to_go } else { 20 } as f64;
        time_remaining / divider + time_increment as f64 / 2.0
    }
}