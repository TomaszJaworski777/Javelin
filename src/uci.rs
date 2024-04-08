use std::{collections::HashMap, sync::{mpsc::{self, Sender}, Arc, Mutex}, thread};

use crate::{core::{create_board, Board, MoveList, MoveProvider, Side}, mcts::{Search, SearchParams, SearchRules}};

type CommandFn = Box<dyn Fn(&mut ContextVariables, &[String]) + Send + Sync + 'static>;

struct ContextVariables {
    board: Board,
    interruption_channel: Option<Sender<()>>,
    search_active: Arc<Mutex<bool>>
}

impl ContextVariables {
    fn new() -> Self {
        ContextVariables {
            board: create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            interruption_channel: None,
            search_active: Arc::new(Mutex::new(false))
        }
    }
}

pub struct Uci {
    commands: HashMap<String, CommandFn>,
    context: ContextVariables,
}

#[allow(unused_variables)]
impl Uci {
    pub fn new() -> Self {
        let mut uci = Uci { 
            commands: HashMap::new(),
            context: ContextVariables::new(),
        };

        uci.add_command("uci", Uci::uci_command);
        uci.add_command("isready", Uci::is_ready_command);
        uci.add_command("ucinewgame", Uci::new_game_command);
        uci.add_command("position", Uci::position_command);
        uci.add_command("draw", Uci::draw_board_command);
        uci.add_command("go", Uci::go_command);
        uci.add_command("stop", Uci::stop_search_command);

        uci
    }

    pub fn print_raport(search_params: &SearchParams, pv_line: String){
        let depth = search_params.get_avg_depth();
        let seldepth = search_params.max_depth;
        let time = search_params.time_passed;
        let nodes = search_params.nodes;
        let nps = ((nodes * 1000) as f64 / (time as f64).max(0.482)) as u64;
        println!("info depth {depth} seldepth {seldepth} score cp 0 time {time} nodes {nodes} nps {nps} pv {pv_line}");
    }

    pub fn execute_command(&mut self, command_name: &str, args: &[String]) {
        if let Some(command) = self.commands.get(command_name) {
            command(&mut self.context, args);
        }
    }

    fn add_command<F>(&mut self, name: &str, action: F)
    where
        F: Fn(&mut ContextVariables, &[String]) + Send + Sync + 'static,
    {
        self.commands.insert(name.to_string(), Box::new(action));
    }

    fn uci_command(context: &mut ContextVariables, args: &[String]) {
        println!("id name Javelin v{}", env!("CARGO_PKG_VERSION"));
        println!("id author Tomasz Jaworski");
        println!("uciok");
    }

    fn is_ready_command(context: &mut ContextVariables, args: &[String]) {
        println!("readyok");
    }

    fn new_game_command(context: &mut ContextVariables, args: &[String]) {
        context.board = create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    fn position_command(context: &mut ContextVariables, args: &[String]) {
        let apply_moves = |moves: &[String], board: &mut Board| {
            let mut move_list = MoveList::new();
            MoveProvider::generate_moves(&mut move_list, &board);
        
            for mv_str in moves.iter().skip_while(|&m| m != "moves").skip(1) {
                if let Some(_move) = move_list.iter().find(|&m| m.to_string() == *mv_str) {
                    board.make_move(*_move);
                }
            }
        };

        match args.split_first() {
            Some((first, rest)) if first.as_str() == "startpos" => {
                let mut new_board = create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
                apply_moves(rest, &mut new_board);
                context.board = new_board;
            },
            Some((first, rest)) if first.as_str() == "fen" && rest.len() >= 6 => {
                let fen = rest[..6].join(" ");
                let mut new_board = create_board(&fen);
    
                if rest.len() > 6 {
                    apply_moves(&rest[6..], &mut new_board);
                }

                context.board = new_board;
            },
            _ => return,
        }
    }

    fn draw_board_command(context: &mut ContextVariables, args: &[String]) {
        context.board.draw_board();
    }

    fn go_command(context: &mut ContextVariables, args: &[String]) {
        let mut rules = SearchRules::new();
        let mut timers = (0u64, 0u64, 0u64, 0u64, 0u64);
        
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "infinite" => rules.infinite = true,
                "wtime" | "btime" | "winc" | "binc" | "movestogo" | "depth" | "nodes" | "iterations" | "movetime" if i + 1 < args.len() => {
                    let value = args[i+1].parse().unwrap_or_default();
                    match args[i].as_str() {
                        "wtime" => timers.0 = value,
                        "btime" => timers.1 = value,
                        "winc" => timers.2 = value,
                        "binc" => timers.3 = value,
                        "movestogo" => timers.4 = value,
                        "depth" => rules.max_depth = value as u32,
                        "nodes" => rules.max_nodes = value as u32,
                        "iterations" => rules.max_iterations = value as u32,
                        "movetime" => rules.time_for_move = value,
                        _ => {},
                    }
                    i += 1;
                },
                _ => {},
            }
            i += 1;
        }
        let (time, increment, moves_to_go) = if context.board.side_to_move == Side::WHITE { 
            (timers.0, timers.2, timers.4) } else { (timers.1, timers.3, timers.4) };
        if time > 0 {
            rules.time_for_move = SearchRules::calculate_time(time, increment, moves_to_go)
        }

        let (sender, reciever) = mpsc::channel::<()>();
        context.interruption_channel = Some(sender);
        let board = context.board;
        let rules_final = rules;
        *context.search_active.lock().unwrap() = true;
        let search_active_clone = Arc::clone(&context.search_active);
        thread::spawn(move || {
            let best_move = Search::new(&board, &reciever).run(&rules_final);
            println!("bestmove {}", best_move.to_string());
            *search_active_clone.lock().unwrap() = false;
        });
    }

    fn stop_search_command(context: &mut ContextVariables, _args: &[String]) {
        if !*context.search_active.lock().unwrap() {
            return;
        }

        if let Some(sender) = &context.interruption_channel {
            sender.send(()).expect("Failed to send stop signal");
        }
    }
}
