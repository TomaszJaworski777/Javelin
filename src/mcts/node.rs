use std::ops::Range;

use crate::core::Move;

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

    pub fn print_node(&self, prefix: &str, reverse_q: bool) {
        let move_str = if self.mv == Move::NULL {
            "root".to_string()
        } else {
            format!("{}. {}", self.index, self.mv.to_string())
        };
        println!(
            "{}{} Q({:.2}%) N({}) P({:.2}%)",
            prefix,
            move_str,
            if reverse_q { 1.0 - self.avg_value() } else { self.avg_value() } * 100.0,
            self.visit_count,
            self.policy_value * 100.0
        );
    }
}
