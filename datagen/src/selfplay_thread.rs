use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;

use javelin::{create_board, Board, GameResult, MoveList, MoveProvider, Search, SearchRules};

use crate::file_manager::Files;
use crate::structs::{ChessMoveInfo, ChessPolicyData, PieceBoard};
use crate::GenData;

pub struct SelfPlayThread {
    gen_data: Arc<Mutex<GenData>>,
}

impl SelfPlayThread {
    pub fn new(gen_data: Arc<Mutex<GenData>>) -> Self {
        Self { gen_data }
    }

    pub fn run(&self, nodes: u32) {
        let gen_data_clone = self.gen_data.clone();
        thread::spawn(move || {
            let mut current_board = get_new_board();
            let mut game_result = GameResult::None;
            let mut temp = Files::new();
            loop {
                let mut search = Search::new(&current_board, None);
                let mut rules = SearchRules::new();
                rules.max_nodes = nodes;
                let (mv, tree) = search.run::<false>(&rules);

                let mut piece_board = PieceBoard::from_board(&current_board);
                piece_board.score = tree.get_best_node().avg_value();
                piece_board.num = tree[0].children_count as u8;

                //save board to temp
                if !temp.push_value(&piece_board, false) {
                    gen_data_clone.lock().unwrap().value_filtered += 1;
                }

                if piece_board.num <= 104 {
                    let mut policy_data =
                        ChessPolicyData { board: piece_board, moves: [ChessMoveInfo::default(); 104] };

                    for (index, child_index) in tree[0].children().into_iter().enumerate() {
                        policy_data.moves[index] = ChessMoveInfo {
                            mov: tree[child_index].mv.value,
                            visits: tree[child_index].visit_count as u16,
                        };
                    }

                    //save policy to temp
                    if !temp.push_policy(&policy_data, false) {
                        gen_data_clone.lock().unwrap().policy_filtered += 1;
                    }
                }

                current_board.make_move(mv);

                if current_board.is_insufficient_material()
                    || current_board.three_fold()
                    || current_board.half_moves >= 100
                {
                    game_result = GameResult::Draw;
                } else {
                    let mut move_list = MoveList::new();
                    MoveProvider::generate_moves::<false>(&mut move_list, &current_board);

                    if move_list.len() == 0 {
                        game_result = if current_board.is_in_check() {
                            if current_board.side_to_move.current() == 0 {
                                GameResult::Lose(0)
                            } else {
                                GameResult::Win(0)
                            }
                        } else {
                            GameResult::Draw
                        }
                    }
                }

                if game_result != GameResult::None {
                    gen_data_clone.lock().unwrap().games_played += 1;

                    //process end of the game
                    match game_result {
                        GameResult::Draw => gen_data_clone.lock().unwrap().draws += 1,
                        GameResult::Win(_) => gen_data_clone.lock().unwrap().wins += 1,
                        GameResult::Lose(_) => gen_data_clone.lock().unwrap().loses += 1,
                        _ => println!("???"),
                    }

                    //iterate through temps and assign result
                    for item in &mut temp.value_data {
                        item.result = match game_result {
                            GameResult::None => 0,
                            GameResult::Draw => 0,
                            GameResult::Lose(_) => {
                                if item.side_to_move == 0 {
                                    -1
                                } else {
                                    1
                                }
                            }
                            GameResult::Win(_) => {
                                if item.side_to_move == 0 {
                                    1
                                } else {
                                    -1
                                }
                            }
                        }
                    }

                    for item in &mut temp.policy_data {
                        item.board.result = match game_result {
                            GameResult::None => 0,
                            GameResult::Draw => 0,
                            GameResult::Lose(_) => {
                                if item.board.side_to_move == 0 {
                                    -1
                                } else {
                                    1
                                }
                            }
                            GameResult::Win(_) => {
                                if item.board.side_to_move == 0 {
                                    1
                                } else {
                                    -1
                                }
                            }
                        }
                    }

                    //push temps into data
                    {
                        let mut data = gen_data_clone.lock().unwrap();
                        data.files.value_data.append(&mut temp.value_data);
                        data.files.policy_data.append(&mut temp.policy_data);
                    }

                    current_board = get_new_board();
                    game_result = GameResult::None;
                }
            }
        });
    }
}

fn get_new_board() -> Board {
    let mut new_board = create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    new_board = shuffle_board(new_board);
    new_board
}

fn shuffle_board(mut board: Board) -> Board {
    let mut rng = rand::thread_rng();
    for _ in 0..rng.gen_range(8..=9) {
        let mut move_list = MoveList::new();
        MoveProvider::generate_moves::<false>(&mut move_list, &board);

        if move_list.len() == 0 {
            board = get_new_board();
            return shuffle_board(board);
        }

        let mv = move_list[if move_list.len() > 1 { rng.gen_range(0..move_list.len()) } else { 0 }];
        board.make_move(mv);
    }

    let mut move_list = MoveList::new();
    MoveProvider::generate_moves::<false>(&mut move_list, &board);

    if move_list.len() == 0 {
        board = get_new_board();
        return shuffle_board(board);
    }

    board
}