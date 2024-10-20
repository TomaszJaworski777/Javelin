#[derive(Clone, Copy)]
pub struct SearchInfo {
    pub current_iterations: i32,
    pub total_depth: u32,
    pub max_depth: u32,
    pub time_passed: u128,
    pub nodes: u32,
}
impl SearchInfo {
    pub fn new() -> Self {
        Self { current_iterations: -1, total_depth: 0, max_depth: 0, time_passed: 0, nodes: 0 }
    }

    #[inline]
    pub fn get_avg_depth(&self) -> u32 {
        self.total_depth / self.current_iterations.max(1) as u32
    }
}
