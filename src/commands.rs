use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
    thread,
};

use crate::{
    benchmark::Benchmark,
    core::{create_board, Board, MoveList, MoveProvider, Side},
    mcts::{Search, SearchRules, SearchTree},
    options::Options,
    perft::Perft,
};

type CommandFn = Box<dyn Fn(&mut ContextVariables, &[String]) + Send + Sync + 'static>;

struct ContextVariables {
    board: Board,
    previous_board: Arc<Mutex<Board>>,
    search: Arc<Mutex<Search<true>>>,
    interruption_token: Arc<RwLock<bool>>,
    uci_initialized: bool,
}

impl ContextVariables {
    fn new() -> Self {
        let board = create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let interruption_token = Arc::new(RwLock::new(false));
        let search = Arc::new(Mutex::new(Search::new(SearchTree::new(), Some(Arc::clone(&interruption_token)))));
        Self {
            board,
            previous_board: Arc::new(Mutex::new(board)),
            search,
            interruption_token,
            uci_initialized: false,
        }
    }
}

pub struct Commands {
    commands: HashMap<String, CommandFn>,
    context: ContextVariables,
}

#[allow(unused_variables)]
impl Commands {
    pub fn new() -> Self {
        let mut commands = Commands { commands: HashMap::new(), context: ContextVariables::new() };

        commands.add_command("uci", Commands::uci_command);
        commands.add_command("setoption", Commands::set_option_command);
        commands.add_command("isready", Commands::is_ready_command);
        commands.add_command("ucinewgame", Commands::new_game_command);
        commands.add_command("position", Commands::position_command);
        commands.add_command("draw", Commands::draw_board_command);
        commands.add_command("go", Commands::go_command);
        commands.add_command("stop", Commands::stop_search_command);
        commands.add_command("tree", Commands::tree_command);
        commands.add_command("perft", Commands::perft_command);
        commands.add_command("bench", Commands::bench_command);

        commands
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
        Options::print();
        println!("uciok");
        context.uci_initialized = true;
    }

    fn set_option_command(context: &mut ContextVariables, args: &[String]) {
        let mut name: String = String::new();
        let mut value: String = String::new();

        if args.len() != 4 {
            println!("Error: Incorrect number of arguments.");
            return;
        }

        for (index, argument) in args.iter().enumerate() {
            match argument.as_str() {
                "name" => {
                    if index + 1 < args.len() {
                        name = args[index + 1].clone();
                    }
                }
                "value" => {
                    if index + 1 < args.len() {
                        value = args[index + 1].clone();
                    }
                }
                _ => continue,
            }
        }

        Options::set(name, value);
    }

    fn is_ready_command(context: &mut ContextVariables, args: &[String]) {
        println!("readyok");
    }

    fn new_game_command(context: &mut ContextVariables, args: &[String]) {
        context.board = create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        context.search = Arc::new(Mutex::new(Search::new(SearchTree::new(), Some(Arc::clone(&context.interruption_token)))));
    }

    fn position_command(context: &mut ContextVariables, args: &[String]) {
        let apply_moves = |moves: &[String], board: &mut Board| {
            if let Some(start_index) = moves.iter().position(|x| x == "moves") {
                for move_str in &moves[start_index + 1..] {
                    let mut move_list = MoveList::new();
                    MoveProvider::generate_moves::<false>(&mut move_list, board);

                    if let Some(mv) = move_list.iter().find(|&m| m.to_string() == *move_str) {
                        board.make_move(*mv);
                    }
                }
            }
        };

        match args.split_first() {
            Some((first, rest)) if first.as_str() == "startpos" => {
                let mut new_board = create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
                apply_moves(rest, &mut new_board);
                context.board = new_board;
            }
            Some((first, rest)) if first.as_str() == "fen" && rest.len() >= 6 => {
                let fen = rest[..6].join(" ");
                let mut new_board = create_board(&fen);

                if rest.len() > 6 {
                    apply_moves(&rest[6..], &mut new_board);
                }

                context.board = new_board;
            }
            _ => return,
        }
    }

    fn draw_board_command(context: &mut ContextVariables, args: &[String]) {
        context.board.draw_board();
    }

    fn go_command(context: &mut ContextVariables, args: &[String]) {
        let mut rules = SearchRules::new();
        let mut timers = (0u64, 0u64, 0u64, 0u64, 0u64);

        if args.len() == 0 {
            rules.infinite = true;
        }

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "infinite" => rules.infinite = true,
                "wtime" | "btime" | "winc" | "binc" | "movestogo" | "depth" | "nodes" | "iterations" | "movetime"
                    if i + 1 < args.len() =>
                {
                    let value = args[i + 1].parse().unwrap_or_default();
                    match args[i].as_str() {
                        "wtime" => timers.0 = value,
                        "btime" => timers.1 = value,
                        "winc" => timers.2 = value,
                        "binc" => timers.3 = value,
                        "movestogo" => timers.4 = value,
                        "depth" => rules.max_depth = value as u32 + context.search.lock().unwrap().search_info().get_avg_depth(),
                        "nodes" => rules.max_nodes = value as u32,
                        "movetime" => rules.time_for_move = value,
                        _ => {}
                    }
                    i += 1;
                }
                _ => {}
            }
            i += 1;
        }
        let (time, increment, moves_to_go) = if context.board.side_to_move == Side::WHITE {
            (timers.0, timers.2, timers.4)
        } else {
            (timers.1, timers.3, timers.4)
        };
        if time > 0 {
            rules.time_for_move = SearchRules::calculate_time(time, increment, moves_to_go)
        }

        context.search.lock().unwrap().reuse_tree(&context.board, &*context.previous_board.lock().unwrap());

        let board = context.board;
        let rules_final = rules;
        let search_clone = Arc::clone(&context.search);
        let previous_board_clone = Arc::clone(&context.previous_board);
        let uci_initialized = context.uci_initialized;
        *context.interruption_token.write().unwrap() = false;
        thread::spawn(move || {
            let result = if uci_initialized { 
                search_clone.lock().unwrap().run::<false>(rules_final, &board) 
            } else { 
                search_clone.lock().unwrap().run::<true>(rules_final, &board) 
            };
            println!("bestmove {}", result.to_string());
            *previous_board_clone.lock().unwrap() = board;
        });
    }

    fn stop_search_command(context: &mut ContextVariables, args: &[String]) {
        *context.interruption_token.write().unwrap() = true;
    }

    fn tree_command(context: &mut ContextVariables, args: &[String]) {
        match args.len() {
            0 => context.search.lock().unwrap().tree().draw_tree_from_root(1),
            1 => context.search.lock().unwrap().tree().draw_tree_from_root(args[0].parse::<i32>().unwrap()),
            2 => context
                .search
                .lock()
                .unwrap()
                .tree()
                .draw_tree_from_node(args[1].parse::<i32>().unwrap(), args[0].parse::<i32>().unwrap()),
            _ => return,
        }
    }

    fn perft_command(context: &mut ContextVariables, args: &[String]) {
        if args.len() != 1 {
            return;
        }

        Perft::execute::<true>(&context.board, args[0].parse().unwrap_or_default(), true);
    }

    fn bench_command(context: &mut ContextVariables, args: &[String]) {
        if args.len() != 1 {
            return;
        }

        Benchmark::run(args[0].parse().unwrap_or_default());
    }
}
