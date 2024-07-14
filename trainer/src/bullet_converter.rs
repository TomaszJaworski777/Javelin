use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::mem::size_of;
use std::path::Path;
use std::time::Instant;

use bullet::format::ChessBoard;
use datagen::PieceBoard;
use javelin::{Bitboard, Board, Side, Square};

const DATA_PATH: &str = "../../resources/data/value.data";
const OUTPUT_PATH: &str = "../../resources/data/bullet_data.data";

#[allow(unused)]
pub fn convert_file() {
    let input_file_path = Path::new(DATA_PATH);
    let output_file_path = Path::new(OUTPUT_PATH);

    let input_file = File::open(input_file_path).unwrap();
    let input_metadata = input_file.metadata().unwrap();
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(output_file_path).unwrap();
    let mut writer = BufWriter::new(output_file);

    let piece_board_size = size_of::<PieceBoard>();
    let chess_board_size = size_of::<ChessBoard>();

    let mut buffer = vec![0u8; piece_board_size];

    let data_loaded = input_metadata.len() / piece_board_size as u64;

    let mut entry_count = 0;
    let mut data_written = 0;
    let mut data_filtered = 0;
    let mut mate_filtered = 0;

    let mut timer = Instant::now();
    let mut entries_this_second = 0u64;

    while reader.read_exact(&mut buffer).is_ok() {
        let piece_board: PieceBoard = unsafe { std::ptr::read(buffer.as_ptr() as *const _) };
        entry_count += 1;
        entries_this_second += 1;

        if timer.elapsed().as_secs_f32() >= 1.0 {
            let entries_to_go = data_loaded - entry_count;
            let seconds_remaining = entries_to_go / entries_this_second.max(1);
            let minutes = seconds_remaining / 60;

            print!("Entry {:.2}k/{:.2}k ({entries_this_second}). Current staus: {:.2}k/{:.2}k (written/filtered). Time remaining: {minutes}m {}s\r", 
                entry_count as f32 / 1000.0, 
                data_loaded as f32 / 1000.0, 
                data_written as f32 / 1000.0, 
                data_filtered as f32/ 1000.0, 
                seconds_remaining % 60);
            let _ = std::io::stdout().flush();

            timer = Instant::now();
            entries_this_second = 0;
        }

        if piece_board.score <= 0.0 || piece_board.score >= 1.0 {
            data_filtered += 1;
            mate_filtered += 1;
            continue;
        }

        let board = Board::from_datapack(&convert_to_12_bitboards(piece_board.piece_boards), piece_board.side_to_move);

        let perspective_score = if piece_board.side_to_move == 0 { piece_board.score } else { 1.0 - piece_board.score };
        let score = -(400.0 * (1.0 / perspective_score - 1.0).ln()) as i16;
        let result = (piece_board.result + 1) as f32 / 2.0;

        let bbs = [
            board.get_occupancy_for_side(Side::WHITE).get_value(),
            board.get_occupancy_for_side(Side::BLACK).get_value(),
            board.get_piece_mask_for_both(1).get_value(),
            board.get_piece_mask_for_both(2).get_value(),
            board.get_piece_mask_for_both(3).get_value(),
            board.get_piece_mask_for_both(4).get_value(),
            board.get_piece_mask_for_both(5).get_value(),
            board.get_piece_mask_for_both(6).get_value(),
        ];

        let chess_board = ChessBoard::from_raw(bbs, board.side_to_move.current(), score, result).unwrap();

        let chess_board_bytes = unsafe {
            std::slice::from_raw_parts(
                (&chess_board as *const ChessBoard) as *const u8,
                chess_board_size,
            )
        };

        writer.write_all(chess_board_bytes).unwrap();
        data_written += 1;
    }

    println!("File converted!");
    println!("Data loaded: {}", data_loaded);
    println!("Total entries: {}", entry_count);
    println!("Entries written: {}", data_written);
    println!("Entries filtered: {}", data_filtered);
    println!(" - Mate score: {}", mate_filtered);
}

fn convert_to_12_bitboards(board: [Bitboard; 4]) -> [Bitboard; 12] {
    let mut result = [Bitboard::EMPTY; 12];
    for square_index in 0..64 {
        let square = Square::from_raw(square_index);
        let piece_index: usize = (if board[0].get_bit(square) { 1 } else { 0 }
            | if board[1].get_bit(square) { 2 } else { 0 }
            | if board[2].get_bit(square) { 4 } else { 0 })
            + if board[3].get_bit(square) { 6 } else { 0 };
        if piece_index == 0 {
            continue;
        }
        result[piece_index - 1].set_bit(square);
    }
    result
}
