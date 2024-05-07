use std::ops::Range;
use crate::core::Move;
use colored::*;
use super::SearchTree;

#[derive(Clone, Copy, PartialEq)]
pub enum GameResult {
    None,
    Lose(u8),
    Draw,
    Win(u8),
}

#[derive(Clone, Copy)]
pub struct Node {
    pub index: u32,
    pub total_value: f32,
    pub visit_count: u32,
    pub first_child_index: u32,
    pub children_count: u32,
    pub policy_value: f32,
    pub result: GameResult,
    pub mv: Move,
}
impl Node {
    pub fn new(mv: Move) -> Self {
        Self {
            index: 0,
            total_value: 0.0,
            visit_count: 0,
            first_child_index: 0,
            children_count: 0,
            policy_value: 0.0,
            result: GameResult::None,
            mv,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children_count == 0
    }

    pub fn is_terminal(&self) -> bool {
        self.result != GameResult::None
    }

    pub fn children(&self) -> Range<u32> {
        let end_index = self.first_child_index + self.children_count;
        self.first_child_index..end_index
    }

    pub fn avg_value(&self) -> f32 {
        if self.visit_count == 0 {
            return 0.5;
        }
        self.total_value / self.visit_count as f32
    }

    pub fn all_children_lost(&self, tree: &SearchTree) -> Option<u8> {
        let lose_values =
            self.children().filter_map(
                |child_index| {
                    if let GameResult::Lose(n) = tree[child_index].result {
                        Some(n)
                    } else {
                        None
                    }
                },
            );

        let min_value = lose_values.min();

        if min_value.is_some()
            && self.children().all(|child_index| matches!(tree[child_index].result, GameResult::Lose(_)))
        {
            min_value
        } else {
            None
        }
    }

    pub fn all_children_draw(&self, tree: &SearchTree) -> bool {
        self.children().all(|child_index| matches!(tree[child_index].result, GameResult::Draw))
    }

    pub fn print_node(&self, prefix: &str, reverse_q: bool, heat_min_value: f32, heat_max_value: f32, has_promotion: bool) {
        let move_str = if self.mv == Move::NULL {
            "root".truecolor(192,210,255).to_string()
        } else {
            format!("{:<4} {}", self.index.to_string() + ".", self.mv.to_string().truecolor(192,210,255))
        };
        let q_value = if reverse_q { 1.0 - self.avg_value() } else { self.avg_value() } * 100.0;
        let q_text = format!("Q({})", heat_color(format!("{:.2}%", q_value).as_str(), q_value, 0.0, 100.0));
        let n_text = format!("N({})", self.visit_count.to_string().truecolor(192,210,255).to_string());
        let p_text = format!("P({})", heat_color(format!("{:.2}%", self.policy_value * 100.0).as_str(), self.policy_value, heat_min_value, heat_max_value));

        if self.mv == Move::NULL
        {
            println!(
                "{}{:<28}{:<31}{:<35}",
                prefix,
                move_str,
                q_text,
                n_text
            );
        }
        else if has_promotion {
            println!(
                "{}{:<34}{:<31}{:<35}{}",
                prefix,
                move_str,
                q_text,
                n_text,
                p_text
            );
        } else {
            println!(
                "{}{:<33}{:<31}{:<35}{}",
                prefix,
                move_str,
                q_text,
                n_text,
                p_text
            );
        }
    }
}

fn heat_color(content: &str, value: f32, min_value: f32, max_value: f32) -> String {
    let scalar = ( value - min_value ) / ( max_value - min_value );
    let r = (255.0 * (1.0 - scalar)) as u8;
    let g = (255.0 * scalar) as u8;
    content.truecolor(r, g, if r < 100 || g < 100 { 10 } else { 0 }).to_string()
}
