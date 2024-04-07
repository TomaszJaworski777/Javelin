mod core;
mod eval;
mod mcts;
mod perft;
mod uci;

use core::create_board;
use perft::Perft;
use uci::Uci;

fn main() {
    let mut uci = Uci::new();
    uci.execute_command("uci", &[]);

    Perft::execute::<false>(&create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"), 6, true);
}
 