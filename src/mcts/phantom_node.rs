use crate::{core::Move, mcts::GameResult};
use colored::*;

#[derive(Clone, Copy, PartialEq)]
pub struct PhantomNode {
    node_index: i32,
    mv: Move,
    policy: i16,
    visits: u32,
    total_score: f32,
    total_score_squared: f32,
}
#[allow(unused)]
impl PhantomNode {
    #[inline]
    pub fn new(node_index: i32, mv: Move, policy: f32) -> Self {
        Self { node_index, mv, policy: (policy * f32::from(i16::MAX)) as i16, total_score: 0.0, visits: 0, total_score_squared: 0.0 }
    }

    #[inline]
    pub fn index(&self) -> i32 {
        self.node_index
    }

    #[inline]
    pub fn set_index(&mut self, index: i32) {
        self.node_index = index
    }

    #[inline]
    pub fn mv(&self) -> Move {
        self.mv
    }

    #[inline]
    pub fn policy(&self) -> f32 {
        f32::from(self.policy) / f32::from(i16::MAX)
    }

    #[inline]
    pub fn visits(&self) -> u32 {
        self.visits
    }

    #[inline]
    pub fn total_score(&self) -> f32 {
        self.total_score
    }

    #[inline]
    pub fn avg_score(&self) -> f32 {
        if self.visits == 0 {
            0.5
        } else {
            self.total_score / self.visits as f32
        }
    }

    #[inline]
    pub fn apply_score(&mut self, score: f32) {
        self.visits += 1;
        self.total_score += score;
        self.total_score_squared += score.powi(2);
    }

    #[inline]
    pub fn update_policy(&mut self, new_policy: f32) {
        self.policy = (new_policy * f32::from(i16::MAX)) as i16
    }

    pub fn variance(&self) -> f32 {
        let visits_f = self.visits as f32;
        let var = self.total_score_squared / visits_f - (self.total_score / visits_f).powi(2);
        var.max(0.0)
    }

    pub fn print_node(
        &self,
        prefix: &str,
        is_root: bool,
        heat_min_value: f32,
        heat_max_value: f32,
        has_promotion: bool,
        game_result: GameResult,
    ) {
        let move_str = if is_root {
            "root".truecolor(192, 210, 255).to_string()
        } else {
            format!("{:<6} {}", self.index().to_string() + ".", self.mv.to_string().truecolor(192, 210, 255))
        };

        let get_node_value = || -> f32 {
            match game_result {
                GameResult::None => self.avg_score(),
                GameResult::Draw => 0.5,
                GameResult::Lose(_) => 1.0,
                GameResult::Win(_) => 0.0,
            }
        };

        let q_value = if is_root { 1.0 - get_node_value() } else { get_node_value() } * 100.0;
        let q_text = format!("Q({})", heat_color(format!("{:.2}%", q_value).as_str(), q_value, 0.0, 100.0));
        let n_text = format!("N({})", self.visits().to_string().truecolor(192, 210, 255).to_string());
        let p_text = format!(
            "P({})",
            heat_color(
                format!("{:.2}%", self.policy() * 100.0).as_str(),
                self.policy(),
                heat_min_value,
                heat_max_value
            )
        );
        let t_text = format!(
            "{}",
            match game_result {
                GameResult::None => "",
                GameResult::Draw => "T(D)",
                GameResult::Lose(_) => "T(W)",
                GameResult::Win(_) => "T(L)",
            }
        );

        if is_root {
            println!("{}{:<30}{:<35}{:<35}", prefix, move_str, q_text, n_text);
        } else if has_promotion {
            println!("{}{:<36}{:<35}{:<35}{:<30}{}", prefix, move_str, q_text, n_text, p_text, t_text);
        } else {
            println!("{}{:<35}{:<35}{:<35}{:<30}{}", prefix, move_str, q_text, n_text, p_text, t_text);
        }
    }
}

fn heat_color(content: &str, value: f32, min_value: f32, max_value: f32) -> String {
    let scalar = if min_value == max_value { 0.5 } else { (value - min_value) / (max_value - min_value) };

    let r = (255.0 * (1.0 - scalar)) as u8;
    let g = (255.0 * scalar) as u8;
    content.truecolor(r, g, if r < 100 || g < 100 { 10 } else { 0 }).to_string()
}
