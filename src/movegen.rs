use arrayvec::ArrayVec;

use crate::{board::Board, types::{Move, Square}};

pub type MoveList = ArrayVec<Move, 256>;

pub struct MoveProvider;
impl MoveProvider {
    pub fn generate_moves( move_list: &mut MoveList, board: &Board ){
        //study check table to decide if you can skip most of the move generation
        //generate king moves including castles
        //generate pawn moves
        //generate knight moves
        //generate bishop moves
        //generate rook moves
        //generate queen moves 
    }

    pub fn generate_capture_moves( move_list: &mut MoveList, board: &Board ){
        
    }
}