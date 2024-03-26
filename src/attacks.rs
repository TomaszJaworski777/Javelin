use crate::consts::{FileMask, Side};

pub struct Attacks;
impl Attacks {
    pub fn get_pawn_attacks_for_square( square_index: usize, color: usize ) -> u64{
        ATTACK_TABLES.pawn_attacks[color][square_index]
    }

    pub fn get_knight_attacks_for_square( square_index: usize ) -> u64{
        ATTACK_TABLES.knight_attacks[square_index]
    }

    pub fn get_king_attacks_for_square( square_index: usize ) -> u64{
        ATTACK_TABLES.king_attacks[square_index]
    }
}

struct AttackTables{
    pawn_attacks : [[u64; 64]; 2],
    knight_attacks: [u64; 64],
    king_attacks: [u64; 64]
} 

static ATTACK_TABLES: AttackTables = AttackTables {
    pawn_attacks: AttackTables::PAWN_ATTACKS,
    knight_attacks: AttackTables::KNIGHT_ATTACKS,
    king_attacks: AttackTables::KING_ATTACKS
};

impl AttackTables {
    const PAWN_ATTACKS: [[u64; 64]; 2] = {
        let mut result = [[0; 64]; 2];
        let mut square_index = 0usize;
        while square_index < 64 {
            let bb: u64 = 1u64 << square_index;
            if bb << 9 & !FileMask::A > 0 { result[Side::WHITE][square_index] |= bb << 9 }
            if bb << 7 & !FileMask::H > 0 { result[Side::WHITE][square_index] |= bb << 7 }
            if bb >> 7 & !FileMask::A > 0 { result[Side::BLACK][square_index] |= bb >> 7 }
            if bb >> 9 & !FileMask::H > 0 { result[Side::BLACK][square_index] |= bb >> 9 }
            square_index += 1;
        }
        result
    };
    const KNIGHT_ATTACKS: [u64; 64] = {
        let mut result = [0; 64];
        let mut square_index = 0usize;
        while square_index < 64 {
            let bb: u64 = 1u64 << square_index;
            if bb << 17 & !FileMask::A > 0 { result[square_index] |= bb << 17 }
            if bb << 15 & !FileMask::H > 0 { result[square_index] |= bb << 15 }
            if bb << 10 & !(FileMask::A | FileMask::B) > 0 { result[square_index] |= bb << 10 }
            if bb << 6 & !(FileMask::H | FileMask::G) > 0 { result[square_index] |= bb << 6 }
            if bb >> 17 & !FileMask::H > 0 { result[square_index] |= bb >> 17 }
            if bb >> 15 & !FileMask::A > 0 { result[square_index] |= bb >> 15 }
            if bb >> 10 & !(FileMask::H | FileMask::G) > 0 { result[square_index] |= bb >> 10 }
            if bb >> 6 & !(FileMask::A | FileMask::B) > 0 { result[square_index] |= bb >> 6 }
            square_index += 1;
        }
        result
    };
    const KING_ATTACKS: [u64; 64] = {
        let mut result = [0; 64];
        let mut square_index = 0usize;
        while square_index < 64 {
            let bb: u64 = 1u64 << square_index;
            if bb << 7 & !FileMask::H > 0 { result[square_index] |= bb << 7 }
            result[square_index] |= bb << 8;
            if bb << 9 & !FileMask::A > 0 { result[square_index] |= bb << 9 }
            if bb >> 7 & !FileMask::A > 0 { result[square_index] |= bb >> 7 }
            result[square_index] |= bb >> 8;
            if bb >> 9 & !FileMask::H > 0 { result[square_index] |= bb >> 9 }
            if bb << 1 & !FileMask::A > 0 { result[square_index] |= bb << 1 }
            if bb >> 1 & !FileMask::H > 0 { result[square_index] |= bb >> 1 }
            square_index += 1;
        }
        result
    };
}