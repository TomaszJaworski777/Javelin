mod core_net_struct;
mod value_net;
mod simple_trainer;
mod value_data_loader;
mod policy_data_loader;

use colored::Colorize;
use javelin::{Bitboard, Side, Square};
use simple_trainer::SimpleTrainer;
use tch::{nn::{linear, seq}, Tensor};

fn main() {
    policy_trainer();
}

#[allow(unused)]
fn value_trainer() {
    let mut trainer = SimpleTrainer::new("base_value");
    let mut structure = seq()
    .add(linear(
        trainer.var_store.root() / format!("0"),
        768,
        4,
        Default::default(),
    ))
    .add_fn(move |xs: &Tensor| xs.clamp(0.0, 1.0).pow_tensor_scalar(2))
    .add(linear(
        trainer.var_store.root() / format!("1"),
        4,
        1,
        Default::default(),
    ))
    .add_fn(move |xs: &Tensor| xs.sigmoid());

    trainer.add_structure(structure);
    trainer.change_learning_rate(0.001, 0.5, 5);
    trainer.change_batch_size(16_384);
    trainer.build();

    trainer.run::<true>();
}

#[allow(unused)]
fn policy_trainer() {
    let mut trainer = SimpleTrainer::new("base_policy");
    let mut structure = seq()
    .add(linear(
        trainer.var_store.root() / format!("0"),
        768,
        384,
        Default::default(),
    ))
    .add_fn(move |xs: &Tensor| xs.sigmoid());

    trainer.add_structure(structure);
    trainer.change_learning_rate(0.001, 0.5, 5);
    trainer.change_batch_size(16_384);
    trainer.build();

    trainer.run::<false>();
}

#[allow(unused)]
fn get_piece_tuple(board: &[Bitboard; 12], square: Square) -> (usize, Side) {
    for (index, bitboard) in board.iter().enumerate() {
        if !bitboard.get_bit(square) {
            continue;
        }
        let piece_index = (index % 6) + 1;
        let color = if index >= 6 { Side::BLACK } else { Side::WHITE };
        return (piece_index, color);
    }
    (0, Side::WHITE)
}

#[allow(unused)]
fn draw_board(board: &[Bitboard; 12]) {
    let piece_icons: [[&str; 7]; 2] =
    [[" . ", " P ", " N ", " B ", " R ", " Q ", " K "], [" . ", " p ", " n ", " b ", " r ", " q ", " k "]];
    let mut result = " ------------------------\n".to_string();
    for rank in (0..8).rev() {
        result += "|".to_string().as_str();
        for file in 0..8 {
            let square = Square::from_coords(rank, file);
            let piece_tuple = get_piece_tuple(&board, square);
            if piece_tuple.0 == 0 {
                result += piece_icons[0][usize::from(piece_tuple.0)];
            } else if piece_tuple.1 == Side::BLACK {
                result +=
                    piece_icons[Side::BLACK.current()][piece_tuple.0].blue().to_string().as_str();
            } else {
                result +=
                    piece_icons[Side::WHITE.current()][piece_tuple.0].yellow().to_string().as_str();
            }
        }
        result += "|".to_string().as_str();
        result += "\n".to_string().as_str();
    }
    result += " ------------------------\n".to_string().as_str();
    print!("{}", result);
}