use std::collections::HashMap;

use crate::core::{create_board, Board};

type CommandFn = Box<dyn Fn(&mut ContextVariables, &[String]) + Send + Sync + 'static>;

struct ContextVariables {
    board: Board,
}

impl ContextVariables {
    fn new() -> Self {
        ContextVariables {
            board: create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
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

        uci
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

    fn uci_command(context: &mut ContextVariables, _args: &[String]) {
        println!("id name Javelin v{}", env!("CARGO_PKG_VERSION"));
        println!("id author Tomasz Jaworski");
        println!("uciok");
    }

    fn is_ready_command(context: &mut ContextVariables, _args: &[String]) {
        println!("readyok");
    }

    fn new_game_command(context: &mut ContextVariables, _args: &[String]) {
        context.board = create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    fn position_command(context: &mut ContextVariables, _args: &[String]) {
        if _args.len() > 0 && _args[0] == "startpos" {
            context.board = create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        }
    }
}
