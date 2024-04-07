pub struct SearchParams {
    pub curernt_iterations: u32,
    pub total_depth: u32,
    pub max_depth: u32,
    pub time_passed: u128,
    pub nodes: u32,
}
impl SearchParams {
    pub fn new() -> Self {
        Self { curernt_iterations: 0, total_depth: 0, max_depth: 0, time_passed: 0, nodes: 0 }
    }

    pub fn get_avg_depth(&self) -> u32 {
        self.total_depth / self.curernt_iterations.max(1)
    }
}
