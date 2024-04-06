mod search_tree;

use arrayvec::ArrayVec;
use crate::board::Board;

use self::search_tree::SearchTree;

type SelectionHistory = ArrayVec<u32, 128>;

pub struct Search {
    search_tree: SearchTree,
    root_position: Board
}
impl Search {
    pub fn new( board: &Board ) -> Self{
        Self { 
            search_tree: SearchTree::new(),
            root_position: *board
        } 
    }
}