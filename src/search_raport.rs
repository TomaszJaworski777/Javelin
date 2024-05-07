use colored::*;
use crate::mcts::GameResult;

pub struct SearchRaport;
impl SearchRaport {
    pub fn pretty_report(depth: u32, seldepth: u32, time:u128, nodes: u32, nps:u128, best_score: f32, result:GameResult, pv_line:String) {
        let score_text: String;
        if let GameResult::Win(n) = result {
            score_text = format!("M{n}").as_str().green().to_string();
        } else if let GameResult::Lose(n) = result {
            score_text = format!("M{n}").as_str().red().to_string();
        } else {
            let score = -400.0 * (1.0 / best_score.clamp(0.0, 1.0) - 1.0).ln();
            if score > 0.0 {
                score_text = format!("+{:.2}", score/100.0).as_str().green().to_string();
            }
            else if score < 0.0 {
                score_text = format!("{:.2}", score/100.0).as_str().red().to_string();
            }
            else{
                score_text = "+0.00".white().to_string();
            }
        }

        let time_text: String;
        if time < 1000 {
            time_text = format!("{}ms", time);
        } else  {
            time_text = format!("{:.2}s", time as f32/1000.0);
        }

        let nodes_text: String;
        if nodes < 1000 {
            nodes_text = format!("{}", nodes);
        } else if nodes < 1_000_000 {
            nodes_text = format!("{:.1}k", nodes as f32/1000.0);
        } else {
            nodes_text = format!("{:.1}m", nodes as f32/1_000_000.0);
        }

        let nps_text: String;
        if nps < 1000 {
            nps_text = format!("{}n/s", nps);
        } else if nps < 1_000_000 {
            nps_text = format!("{:.1}kn/s", nps as f32/1000.0);
        } else {
            nps_text = format!("{:.1}mn/s", nps as f32/1_000_000.0);
        }

        println!(
            "   {depth_text:<8}{score_text:<18}{time_text:<10}{nodes_text:<10}{nps_text:<13}{pv_line}", 
            depth_text = format!("{}/{}", depth, seldepth)
        );
    }

    pub fn uci_report(depth: u32, seldepth: u32, time:u128, nodes: u32, nps:u128, best_score: f32, result:GameResult, pv_line:String) {
        let score_text: String;
        if let GameResult::Win(n) = result {
            score_text = format!("mate {n}");
        } else if let GameResult::Lose(n) = result {
            score_text = format!("mate -{n}");
        } else {
            score_text = format!("cp {}", (-400.0 * (1.0 / best_score.clamp(0.0, 1.0) - 1.0).ln()) as i32);
        }
        println!(
            "info depth {depth} seldepth {seldepth} score {score_text} time {time} nodes {nodes} nps {nps} pv {pv_line}"
        );
    }
}