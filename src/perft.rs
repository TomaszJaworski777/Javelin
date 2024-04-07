use std::time::Instant;

use crate::core::{create_board, Board, MoveList, MoveProvider};

pub struct Perft;
impl Perft {
    pub fn execute<const BULK: bool>(board: &Board, depth: u8, log: bool) -> u64 {
        let timer = Instant::now();
        let nodes = Perft::internal_execute::<BULK>(board, if depth < 1 { 1 } else { depth }, log);
        let duration = timer.elapsed().as_secs_f64();
        let nodes_per_second = (nodes as f64 / duration) as u64;

        if log {
            print!("-----------------------------------------------------------\n");
            print!("  Perft ended! {} nodes, {}s, {:.2} Mnps\n", nodes, duration, nodes_per_second as f64 / 1000000f64);
            print!("-----------------------------------------------------------\n");
        }

        nodes
    }

    fn internal_execute<const BULK: bool>(board: &Board, depth: u8, first_iteration: bool) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0u64;

        let mut move_list = MoveList::new();
        MoveProvider::generate_moves(&mut move_list, board);

        if BULK && depth == 1 && !first_iteration {
            return move_list.len() as u64;
        }

        for _move in move_list {
            let mut new_board = *board;
            new_board.make_move(_move);
            let new_nodes = Perft::internal_execute::<BULK>(&new_board, depth - 1, false);
            nodes += new_nodes;

            if first_iteration {
                print!("{} - {}\n", _move.to_string(), new_nodes);
                //new_board.draw_board();
            }
        }

        nodes
    }

    #[allow(dead_code)]
    pub fn perft_test() {
        {
            let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
            let brd = create_board(fen);
            print!("{}\n", fen);
            print!("{}\n\n", if Perft::execute::<true>(&brd, 6, false) == 119060324 { "passed" } else { "not passed" });
        }

        {
            let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
            let brd = create_board(fen);
            print!("{}\n", fen);
            print!("{}\n\n", if Perft::execute::<true>(&brd, 5, false) == 193690690 { "passed" } else { "not passed" });
        }

        {
            let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
            let brd = create_board(fen);
            print!("{}\n", fen);
            print!("{}\n\n", if Perft::execute::<true>(&brd, 7, false) == 178633661 { "passed" } else { "not passed" });
        }

        {
            let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
            let brd = create_board(fen);
            print!("{}\n", fen);
            print!("{}\n\n", if Perft::execute::<true>(&brd, 6, false) == 706045033 { "passed" } else { "not passed" });
        }

        {
            let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
            let brd = create_board(fen);
            print!("{}\n", fen);
            print!("{}\n\n", if Perft::execute::<true>(&brd, 6, false) == 3048196529 { "passed" } else { "not passed" });
        }

        {
            let fen = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
            let brd = create_board(fen);
            print!("{}\n", fen);
            print!("{}\n\n", if Perft::execute::<true>(&brd, 5, false) == 164075551 { "passed" } else { "not passed" });
        }
    }

    #[allow(dead_code)]
    pub fn test_speed() -> u64 {
        let board = create_board("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        let mut node_result = 0u64;
        let mut duration = 0f64;
        for _ in 0..5 {
            let timer = Instant::now();
            node_result += Perft::execute::<true>(&board, 5, false);
            duration += timer.elapsed().as_secs_f64();
        }

        let avg_nodes = node_result / 5;
        let avg_time = duration / 5f64;

        (avg_nodes as f64 / avg_time) as u64
    }
}
