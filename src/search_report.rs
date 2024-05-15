use crate::mcts::{GameResult, SearchInfo, SearchTree};
use colored::*;

pub struct SearchReport;
impl SearchReport {
    pub fn print_report<const PRETTY_PRINT: bool>(
        search_params: &SearchInfo,
        pv_line: String,
        best_score: f32,
        result: GameResult,
        tree: &SearchTree,
    ) -> String {
        let depth = search_params.get_avg_depth();
        let seldepth = search_params.max_depth;
        let time: u128 = search_params.time_passed;
        let iterations = search_params.current_iterations;
        let nps = (iterations as u128) * 1000 / time.max(1);

        if PRETTY_PRINT {
            SearchReport::pretty_report(depth, seldepth, time, iterations, nps, best_score, result, pv_line, tree)
        } else {
            SearchReport::uci_report(depth, seldepth, time, iterations, nps, best_score, result, pv_line, tree)
        }
    }

    fn pretty_report(
        depth: u32,
        seldepth: u32,
        time: u128,
        iterations: i32,
        nps: u128,
        best_score: f32,
        result: GameResult,
        pv_line: String,
        tree: &SearchTree,
    ) -> String {
        let score_text: String;
        if let GameResult::Win(n) = result {
            score_text = format!("-M{n}").as_str().red().to_string();
        } else if let GameResult::Lose(n) = result {
            score_text = format!("+M{n}").as_str().green().to_string();
        } else {
            let score = -400.0 * (1.0 / best_score.clamp(0.0, 1.0) - 1.0).ln();
            if score > 0.0 {
                score_text = format!("+{:.2}", score / 100.0).as_str().green().to_string();
            } else if score < 0.0 {
                score_text = format!("{:.2}", score / 100.0).as_str().red().to_string();
            } else {
                score_text = "+0.00".white().to_string();
            }
        }

        let time_text: String;
        if time < 1000 {
            time_text = format!("{}ms", time);
        } else {
            time_text = format!("{:.2}s", time as f32 / 1000.0);
        }

        let nodes_text: String;
        if iterations < 1000 {
            nodes_text = format!("{}", iterations);
        } else if iterations < 1_000_000 {
            nodes_text = format!("{:.1}k", iterations as f32 / 1000.0);
        } else {
            nodes_text = format!("{:.1}m", iterations as f32 / 1_000_000.0);
        }

        let nps_text: String;
        if nps < 1000 {
            nps_text = format!("{}n/s", nps);
        } else if nps < 1_000_000 {
            nps_text = format!("{:.1}kn/s", nps as f32 / 1000.0);
        } else {
            nps_text = format!("{:.1}mn/s", nps as f32 / 1_000_000.0);
        }

        let usage_permill = (tree.usage() * 100.0) as usize;
        let hashfull_text = format!("{usage_permill}%");

        let result = format!("   {depth_text:<8}{score_text:<18}{time_text:<10}{nodes_text:<10}{nps_text:<13}{hashfull_text:<8}{pv_line}",
        depth_text = format!("{}/{}", depth, seldepth));
        result
    }

    fn uci_report(
        depth: u32,
        seldepth: u32,
        time: u128,
        iterations: i32,
        nps: u128,
        best_score: f32,
        result: GameResult,
        pv_line: String,
        tree: &SearchTree,
    ) -> String {
        let score_text: String;
        if let GameResult::Win(n) = result {
            score_text = format!("mate {n}");
        } else if let GameResult::Lose(n) = result {
            score_text = format!("mate -{n}");
        } else {
            score_text = format!("cp {}", (-400.0 * (1.0 / best_score.clamp(0.0, 1.0) - 1.0).ln()) as i32);
        }
        let usage_permill = (tree.usage() * 1000.0) as usize;
        let result = format!("info depth {depth} seldepth {seldepth} score {score_text} time {time} nodes {iterations} nps {nps} hashfull {usage_permill} pv {pv_line}");
        result
    }
}
