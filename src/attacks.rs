use crate::{bitboard::Bitboard, board::Board, consts::{Piece, Side}, types::Square};
extern crate rand;
use rand::Rng;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct Attacks;
impl Attacks {
    pub fn get_pawn_attacks_for_square( square: Square, color: usize ) -> u64{
        ATTACK_TABLES.lock().unwrap().pawn_attacks[color][square.get_value()]
    }

    pub fn get_knight_attacks_for_square( square: Square) -> u64{
        ATTACK_TABLES.lock().unwrap().knight_attacks[square.get_value()]
    }

    pub fn get_king_attacks_for_square( square: Square) -> u64{
        ATTACK_TABLES.lock().unwrap().king_attacks[square.get_value()]
    }

    pub fn get_bishop_attacks_for_square( square: Square, occupancy: Bitboard ) -> u64{
        let attack_tables = ATTACK_TABLES.lock().unwrap();
        let mut occ = occupancy.get_value();
        occ &= attack_tables.bishop_masks[square.get_value()];
        occ = u64::wrapping_mul(occ, MAGIC_NUMBERS_BISHOP[square.get_value()]);
        occ >>= 64 - BISHOP_OCCUPANCY_COUNT[square.get_value()];
        attack_tables.bishop_attacks[square.get_value()][occ as usize]
    }

    pub fn get_rook_attacks_for_square( square: Square, occupancy: Bitboard ) -> u64{
        let attack_tables = ATTACK_TABLES.lock().unwrap();
        let mut occ = occupancy.get_value();
        occ &= attack_tables.rook_masks[square.get_value()];
        occ = u64::wrapping_mul(occ, MAGIC_NUMBERS_ROOK[square.get_value()]);
        occ >>= 64 - ROOK_OCCUPANCY_COUNT[square.get_value()];
        attack_tables.rook_attacks[square.get_value()][occ as usize]
    }

    pub fn get_queen_attacks_for_square( square: Square, occupancy: Bitboard ) -> u64{
        Attacks::get_bishop_attacks_for_square( square, occupancy ) | Attacks::get_rook_attacks_for_square( square, occupancy )
    }

    pub fn initialize_slider_pieces(){
        let mut attack_tables = ATTACK_TABLES.lock().unwrap();

        for square_index in 0..64 {
            {
                let attack_mask = mask_bishop_attacks(square_index);
                let relevant_bit_count = attack_mask.count_ones() as usize;
                let mut index = 0;
                while index < 1 << relevant_bit_count {
                    let occupancy = generate_occupancy(index, relevant_bit_count, attack_mask);
                    let magic_index = u64::wrapping_mul(occupancy, MAGIC_NUMBERS_BISHOP[square_index]) >> (64 - relevant_bit_count);
                    attack_tables.bishop_attacks[square_index][magic_index as usize] = generate_bishop_attacks(square_index, occupancy);
                    index += 1;
                }
            }

            {
                let attack_mask = mask_rook_attacks(square_index);
                let relevant_bit_count = attack_mask.count_ones() as usize;
                let mut index = 0;
                while index < 1 << relevant_bit_count {
                    let occupancy = generate_occupancy(index, relevant_bit_count, attack_mask);
                    let magic_index = u64::wrapping_mul(occupancy, MAGIC_NUMBERS_ROOK[square_index]) >> (64 - relevant_bit_count);
                    attack_tables.rook_attacks[square_index][magic_index as usize] = generate_rook_attacks(square_index, occupancy);
                    index += 1;
                }
            }
        }
    }

    pub fn generate_checkers_mask( board: &Board ) -> Bitboard{
        let mut result = 0u64;
        let occupancy_mask = board.get_occupancy();
        let square = board.get_piece_mask(Piece::KING, board.side_to_move).ls1b_square();
        let attacker_color = 1 - board.side_to_move;

        result |= Attacks::get_bishop_attacks_for_square(square, occupancy_mask) & board.get_piece_mask(Piece::BISHOP, attacker_color).or(board.get_piece_mask(Piece::QUEEN, attacker_color)).get_value();
        result |= Attacks::get_knight_attacks_for_square(square) & board.get_piece_mask(Piece::KNIGHT, attacker_color).get_value();
        result |= Attacks::get_rook_attacks_for_square(square, occupancy_mask) & board.get_piece_mask(Piece::ROOK, attacker_color).or(board.get_piece_mask(Piece::QUEEN, attacker_color)).get_value();
        result |= Attacks::get_pawn_attacks_for_square(square, 1-attacker_color) & board.get_piece_mask(Piece::PAWN, attacker_color).get_value();
        result |= Attacks::get_king_attacks_for_square(square) & board.get_piece_mask(Piece::KING, attacker_color).get_value();
        Bitboard::from_raw(result)
    }

    pub fn generate_ortographic_pins_mask( board: &Board ) -> Bitboard{
        Bitboard::EMPTY
    }

    pub fn generate_diagonal_pins_mask( board: &Board ) -> Bitboard{
        Bitboard::EMPTY
    }
}

struct AttackTables{
    pawn_attacks : [[u64; 64]; 2],
    knight_attacks: [u64; 64],
    king_attacks: [u64; 64],
    bishop_masks: [u64; 64],
    bishop_attacks: Vec<Vec<u64>>,
    rook_masks: [u64; 64],
    rook_attacks: Vec<Vec<u64>>,
} 

static ATTACK_TABLES: Lazy<Mutex<AttackTables>> = Lazy::new(|| {
    Mutex::new(AttackTables {
        pawn_attacks: AttackTables::PAWN_ATTACKS,
        knight_attacks: AttackTables::KNIGHT_ATTACKS,
        king_attacks: AttackTables::KING_ATTACKS,
        bishop_masks: AttackTables::BISHOP_MASKS,
        bishop_attacks: vec![vec![0; 512]; 64],
        rook_masks: AttackTables::ROOK_MASKS,
        rook_attacks: vec![vec![0; 4096]; 64],
    })
});

impl AttackTables {
    const PAWN_ATTACKS: [[u64; 64]; 2] = {
        let mut result = [[0; 64]; 2];
        let mut square_index = 0usize;
        while square_index < 64 {
            let bb = Bitboard::from_raw(1u64 << square_index);
            if !Bitboard::FILE_A.inverse().and(bb.shift_left(9)).is_empty() { result[Side::WHITE][square_index] |= bb.shift_left(9).get_value() }
            if !Bitboard::FILE_H.inverse().and(bb.shift_left(7)).is_empty() { result[Side::WHITE][square_index] |= bb.shift_left(7).get_value() }
            if !Bitboard::FILE_A.inverse().and(bb.shift_right(7)).is_empty() { result[Side::BLACK][square_index] |= bb.shift_right(7).get_value() }
            if !Bitboard::FILE_H.inverse().and(bb.shift_right(9)).is_empty() { result[Side::BLACK][square_index] |= bb.shift_right(9).get_value() }
            square_index += 1;
        }
        result
    };
    const KNIGHT_ATTACKS: [u64; 64] = {
        let mut result = [0; 64];
        let mut square_index = 0usize;
        while square_index < 64 {
            let bb = Bitboard::from_raw(1u64 << square_index);
            if !Bitboard::FILE_A.inverse().and(bb.shift_left(17)).is_empty() { result[square_index] |= bb.shift_left(17).get_value() }
            if !Bitboard::FILE_H.inverse().and(bb.shift_left(15)).is_empty() { result[square_index] |= bb.shift_left(15).get_value() }
            if !Bitboard::FILE_A.or(Bitboard::FILE_B).inverse().and(bb.shift_left(10)).is_empty() { result[square_index] |= bb.shift_left(10).get_value() }
            if !Bitboard::FILE_H.or(Bitboard::FILE_G).inverse().and(bb.shift_left(6)).is_empty() { result[square_index] |= bb.shift_left(6).get_value() }
            if !Bitboard::FILE_H.inverse().and(bb.shift_right(17)).is_empty() { result[square_index] |= bb.shift_right(17).get_value() }
            if !Bitboard::FILE_A.inverse().and(bb.shift_right(15)).is_empty() { result[square_index] |= bb.shift_right(15).get_value()}
            if !Bitboard::FILE_H.or(Bitboard::FILE_G).inverse().and(bb.shift_right(10)).is_empty() { result[square_index] |= bb.shift_right(10).get_value() }
            if !Bitboard::FILE_A.or(Bitboard::FILE_B).inverse().and(bb.shift_right(6)).is_empty() { result[square_index] |= bb.shift_right(6).get_value() }
            square_index += 1;
        }
        result
    };
    const KING_ATTACKS: [u64; 64] = {
        let mut result = [0; 64];
        let mut square_index = 0usize;
        while square_index < 64 {
            let bb = Bitboard::from_raw(1u64 << square_index);
            if !Bitboard::FILE_H.inverse().and(bb.shift_left(7)).is_empty() { result[square_index] |= bb.shift_right(7).get_value() }
            result[square_index] |= bb.shift_left(8).get_value();
            if !Bitboard::FILE_A.inverse().and(bb.shift_left(9)).is_empty() { result[square_index] |= bb.shift_left(9).get_value() }
            if !Bitboard::FILE_A.inverse().and(bb.shift_right(7)).is_empty() { result[square_index] |= bb.shift_right(7).get_value() }
            result[square_index] |= bb.shift_right(8).get_value();
            if !Bitboard::FILE_H.inverse().and(bb.shift_right(9)).is_empty() { result[square_index] |= bb.shift_right(9).get_value() }
            if !Bitboard::FILE_A.inverse().and(bb.shift_left(1)).is_empty() { result[square_index] |= bb.shift_left(1).get_value() }
            if !Bitboard::FILE_H.inverse().and(bb.shift_right(1)).is_empty() { result[square_index] |= bb.shift_right(1).get_value() }
            square_index += 1;
        }
        result
    };
    const BISHOP_MASKS: [u64; 64] = {
        let mut result = [0; 64];
        let mut square_index = 0usize;
        while square_index < 64 {
            result[square_index] = mask_bishop_attacks(square_index);
            square_index += 1;
        }
        result
    };
    const ROOK_MASKS: [u64; 64] = {
        let mut result = [0; 64];
        let mut square_index = 0usize;
        while square_index < 64 {
            result[square_index] = mask_rook_attacks(square_index);
            square_index += 1;
        }
        result
    };
}

const fn mask_bishop_attacks( square_index: usize ) -> u64 {
    let mut result: u64 = 0;
    let bishop_position = (square_index as i32 / 8, square_index as i32 % 8);

    let mut rank = bishop_position.0 + 1;
    let mut file = bishop_position.1 + 1;
    while rank < 7 && file < 7 { result |= 1u64 << rank * 8 + file; rank += 1; file += 1; }

    rank = bishop_position.0 - 1;
    file = bishop_position.1 + 1;
    while rank > 0 && file < 7 { result |= 1u64 << rank * 8 + file; rank -= 1; file += 1; }

    rank = bishop_position.0 - 1;
    file = bishop_position.1 - 1;
    while rank > 0 && file > 0 { result |= 1u64 << rank * 8 + file; rank -= 1; file -= 1; }

    rank = bishop_position.0 + 1;
    file = bishop_position.1 - 1;
    while rank < 7 && file > 0 { result |= 1u64 << rank * 8 + file; rank += 1; file -= 1; }

    result
}

fn generate_bishop_attacks( square_index: usize, occupancy: u64 ) -> u64 {
    let mut result: u64 = 0;
    let bishop_position = (square_index as i32 / 8, square_index as i32 % 8);

    let mut rank = bishop_position.0 + 1;
    let mut file = bishop_position.1 + 1;
    while rank < 8 && file < 8 { result |= 1u64 << rank * 8 + file; if 1u64 << rank * 8 + file & occupancy > 0 { break } rank += 1; file += 1; }

    rank = bishop_position.0 - 1;
    file = bishop_position.1 + 1;
    while rank >= 0 && file < 8 { result |= 1u64 << rank * 8 + file; if 1u64 << rank * 8 + file & occupancy > 0 { break } rank -= 1; file += 1; }

    rank = bishop_position.0 - 1;
    file = bishop_position.1 - 1;
    while rank >= 0 && file >= 0 { result |= 1u64 << rank * 8 + file; if 1u64 << rank * 8 + file & occupancy > 0 { break }rank -= 1; file -= 1; }

    rank = bishop_position.0 + 1;
    file = bishop_position.1 - 1;
    while rank < 8 && file >= 0 { result |= 1u64 << rank * 8 + file; if 1u64 << rank * 8 + file & occupancy > 0 { break } rank += 1; file -= 1; }

    result
}

const fn mask_rook_attacks( square_index: usize ) -> u64 {
    let mut result: u64 = 0;
    let bishop_position = (square_index as i32 / 8, square_index as i32 % 8);
    
    let mut rank = bishop_position.0 + 1;
    let mut file = bishop_position.1;
    while rank < 7 { result |= 1u64 << rank * 8 + file; rank += 1; }

    rank = bishop_position.0 - 1;
    file = bishop_position.1;
    while rank > 0 { result |= 1u64 << rank * 8 + file; rank -= 1; }

    rank = bishop_position.0;
    file = bishop_position.1 + 1;
    while file < 7 { result |= 1u64 << rank * 8 + file; file += 1; }

    rank = bishop_position.0;
    file = bishop_position.1 - 1;
    while file > 0 { result |= 1u64 << rank * 8 + file; file -= 1; }

    result
}

fn generate_rook_attacks( square_index: usize, occupancy: u64 ) -> u64 {
    let mut result: u64 = 0;
    let bishop_position = (square_index as i32 / 8, square_index as i32 % 8);
    
    let mut rank = bishop_position.0 + 1;
    let mut file = bishop_position.1;
    while rank < 8 { result |= 1u64 << rank * 8 + file; if 1u64 << rank * 8 + file & occupancy > 0 { break } rank += 1; }

    rank = bishop_position.0 - 1;
    file = bishop_position.1;
    while rank >= 0 { result |= 1u64 << rank * 8 + file; if 1u64 << rank * 8 + file & occupancy > 0 { break } rank -= 1; }

    rank = bishop_position.0;
    file = bishop_position.1 + 1;
    while file < 8 { result |= 1u64 << rank * 8 + file; if 1u64 << rank * 8 + file & occupancy > 0 { break } file += 1; }

    rank = bishop_position.0;
    file = bishop_position.1 - 1;
    while file >= 0 { result |= 1u64 << rank * 8 + file; if 1u64 << rank * 8 + file & occupancy > 0 { break } file -= 1; }

    result
}

fn generate_occupancy( index: usize, bit_count: usize, attack_mask: u64 ) -> u64 {
    let mut result = 0u64;
    let mut attack_mask_cpy = attack_mask;
    let mut count_index = 0u16;
    while count_index < bit_count as u16 {
        let square_index: u8 = attack_mask_cpy.trailing_zeros() as u8;
        attack_mask_cpy &= !(1 << square_index);
        if index & (1usize << count_index) > 0 {
            result |= 1u64 << square_index;
        }

        count_index += 1;
    }

    result
}

pub const BISHOP_OCCUPANCY_COUNT: [usize; 64] = {
    let mut result = [0; 64];
    let mut rank = 0;
    while rank < 8 {
        let mut file = 0;
        while file < 8 {
            let square_index = rank * 8 + file;
            result[square_index] = mask_bishop_attacks(square_index).count_ones() as usize;
            file += 1;
        }
        rank += 1;
    }
    result
};

pub const ROOK_OCCUPANCY_COUNT: [usize; 64] = {
    let mut result = [0; 64];
    let mut rank = 0;
    while rank < 8 {
        let mut file = 0;
        while file < 8 {
            let square_index = rank * 8 + file;
            result[square_index] = mask_rook_attacks(square_index).count_ones() as usize;
            file += 1;
        }
        rank += 1;
    }
    result
};

fn get_low_ones_random_u64() -> u64 {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..=std::u32::MAX) as u64 & 0xFFFF | (rng.gen_range(0..=std::u32::MAX) as u64 & 0xFFFF) << 16 | (rng.gen_range(0..=std::u32::MAX) as u64 & 0xFFFF) << 32 | (rng.gen_range(0..=std::u32::MAX) as u64 & 0xFFFF) << 48) & 
    (rng.gen_range(0..=std::u32::MAX) as u64 & 0xFFFF | (rng.gen_range(0..=std::u32::MAX) as u64 & 0xFFFF) << 16 | (rng.gen_range(0..=std::u32::MAX) as u64 & 0xFFFF) << 32 | (rng.gen_range(0..=std::u32::MAX) as u64 & 0xFFFF) << 48) & 
    (rng.gen_range(0..=std::u32::MAX) as u64 & 0xFFFF | (rng.gen_range(0..=std::u32::MAX) as u64 & 0xFFFF) << 16 | (rng.gen_range(0..=std::u32::MAX) as u64 & 0xFFFF) << 32 | (rng.gen_range(0..=std::u32::MAX) as u64 & 0xFFFF) << 48)
}

pub fn find_magic_number( square_index: usize, relevent_bit_count: usize, is_bishop: bool ) -> u64 {
    let mut occupancies = [0u64; 4096];
    let mut attacks = [0u64; 4096];
    let attack_mask = if is_bishop { mask_bishop_attacks(square_index) } else { mask_rook_attacks(square_index) };
    let occupancy_count = 1 << relevent_bit_count;
    let mut occupancy_index = 0;
    while occupancy_index < occupancy_count {
        occupancies[occupancy_index] = generate_occupancy(occupancy_index, relevent_bit_count, attack_mask);
        attacks[occupancy_index] = if is_bishop { generate_bishop_attacks(square_index, occupancies[occupancy_index]) } 
                                             else { generate_rook_attacks(square_index, occupancies[occupancy_index]) };
        
        occupancy_index += 1;
    }

    let mut used_attacks: [u64; 4096];
    loop {
        let magic_number = get_low_ones_random_u64();
        if (u64::wrapping_mul( attack_mask, magic_number ) & 0xFF00000000000000).count_ones() < 6 {
            continue;
        }


        used_attacks = [0u64; 4096];
        let mut occupancy_index = 0usize;
        let mut fail_flag = false;
        while occupancy_index < occupancy_count {
            let magic_index = u64::wrapping_mul(occupancies[occupancy_index], magic_number) as usize >> (64 - relevent_bit_count);
            if used_attacks[magic_index] == 0 {
                used_attacks[magic_index] = attacks[occupancy_index];
            }
            else if used_attacks[magic_index] != attacks[occupancy_index]{
                fail_flag = true;
                break;
            }
            occupancy_index += 1;
        }

        if !fail_flag {
            return magic_number;
        }
    }
}

pub fn test_magic_number( square_index: usize, is_bishop: bool ) -> bool {
    let mut occupancies = [0u64; 4096];
    let mut attacks = [0u64; 4096];
    let attack_mask = if is_bishop { mask_bishop_attacks(square_index) } else { mask_rook_attacks(square_index) };
    let relevent_bit_count = if is_bishop { BISHOP_OCCUPANCY_COUNT[square_index] } else { ROOK_OCCUPANCY_COUNT[square_index] };
    let occupancy_count = 1 << relevent_bit_count;
    let mut occupancy_index = 0;
    while occupancy_index < occupancy_count {
        occupancies[occupancy_index] = generate_occupancy(occupancy_index, relevent_bit_count, attack_mask);
        attacks[occupancy_index] = if is_bishop { generate_bishop_attacks(square_index, occupancies[occupancy_index]) } 
                                             else { generate_rook_attacks(square_index, occupancies[occupancy_index]) };
        
        occupancy_index += 1;
    }

    let magic_number = if is_bishop { MAGIC_NUMBERS_BISHOP[square_index] } else { MAGIC_NUMBERS_ROOK[square_index] };
    let mut used_attacks = [0u64; 4096];
    occupancy_index = 0usize;
    let mut fail_flag = false;
    while occupancy_index < occupancy_count {
        let magic_index = u64::wrapping_mul(occupancies[occupancy_index], magic_number) as usize >> (64 - relevent_bit_count);
        if used_attacks[magic_index] == 0 {
            used_attacks[magic_index] = attacks[occupancy_index];
        }
        else if used_attacks[magic_index] != attacks[occupancy_index]{
            fail_flag = true;
            break;
        }
        occupancy_index += 1;
    }

    return !fail_flag;
}

const MAGIC_NUMBERS_BISHOP: [u64; 64] = [
    9300092178686681120,
    1284830893973760,
    2322997520105472,
    16142031364873674789,
    10383348832699154706,
    571763293421568,
    37726495118197760,
    1513231473652670722,
    40550006146990185,
    873700543932137730,
    36037870288505856,
    431188982368272,
    1155210765395821056,
    11538293718411908608,
    4611721787053966849,
    103589390848170272,
    1125968899098624,
    144680358661721088,
    11259553153024529,
    10133272101128193,
    73751202732572676,
    144679238632472832,
    2357915965813425297,
    401383670122021888,
    13528392142225729,
    4643215615211930112,
    9226802530447557664,
    1302666467161997954,
    1306326466426847232,
    2253998841200772,
    9223935538715955216,
    4611977389012961280,
    1161101459345408,
    5630633405878272,
    154573777173479968,
    5224739618297217088,
    184790590020518016,
    141291540840712,
    4621296042111943168,
    9278545841721754664,
    13814550243590400,
    757176487873905668,
    2598717998437009408,
    2344123889522575360,
    360290349769303040,
    14053484853547533328,
    9227878118977438752,
    5102361295591936,
    5233754530306591776,
    4689658989384957952,
    1161642645719051,
    2252351784355840,
    2337004390424,
    1190112502864707589,
    290499785468691593,
    2387190454312566784,
    1235149585505599557,
    4683745820179825441,
    18014407116507136,
    1741698094928005,
    144749056665649409,
    576461028523640968,
    74921813755137,
    18085875364200714,
];

const MAGIC_NUMBERS_ROOK: [u64; 64] = [
    9259400973461241857,
    234187460333015040,
    36063981659521032,
    2377918195574046724,
    1080868867234332928,
    72061992118059016,
    180144534867411456,
    72058693558158370,
    5260345103070806016,
    1378171992426954752,
    13835199342794776576,
    90353536244130048,
    1155314059089281152,
    583356906421125632,
    562984346714500,
    585608691194020096,
    1188951126274211904,
    40550263712383040,
    144749606589170949,
    576762018642657345,
    4613938094192984576,
    1126449729896576,
    144116291882713600,
    1128099206989892,
    4908959330109243397,
    5764677945467601024,
    35186520621184,
    166650782695882760,
    4408784453760,
    9549885211018265600,
    18028709342085744,
    4423816397473,
    15024008631798472704,
    144185694263185412,
    9799938353053839360,
    4614078624457295873,
    578721350366004224,
    704795551728640,
    1729663887059452416,
    576461303166534673,
    9511672783898181668,
    9259488795341373440,
    153123487114919972,
    4503634054234176,
    144396697438584836,
    2199090397312,
    2395916444903931912,
    281476058906626,
    288275458347631104,
    14001693577961277760,
    1585284936444020224,
    5764748329242591872,
    22799490427785472,
    140746078552192,
    81346276859576576,
    325398273679442432,
    35257390760450,
    15908851192709121,
    8076117492512065602,
    148746468910469121,
    4653907677319540842,
    281509370265601,
    162130969700081796,
    1445673626624869378,
];