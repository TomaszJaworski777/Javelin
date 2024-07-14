use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::mem::size_of;
use std::path::Path;
use std::time::Instant;

use datagen::PieceBoard;
use javelin::{Bitboard, Board, Square};

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
        let fen = board.get_fen();

        let score = -(400.0 * (1.0 / piece_board.score - 1.0).ln()) as i16;
        let result = piece_board.result;

        writeln!(writer, "{} | {} | {}", fen, score, result).unwrap();
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
