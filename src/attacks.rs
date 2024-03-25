pub struct Attacks {
    pawn_attacks : [[Bitboard64; 64]; 2]
}

impl Attacks {
    pub fn create_attacks() -> Attacks {
        return Attacks {
            pawn_attacks: [[Bitboard64::NULL; 64]; 2]
        }
    }

    pub fn get_pawn_attack_bitboard( &self, square: usize, color: usize ) -> Bitboard64 {
        return self.pawn_attacks[color][square];
    }
}