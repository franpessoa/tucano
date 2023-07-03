pub mod fen;
pub struct Board {
    board: BitBoards,
    state: BoardState
}

pub struct BoardState {
    moves: u16,
    turn: bool, // 1 -> white, 0 -> black
    castle: CastleState
}

pub struct CastleState {
    white_oo_castle: bool,
    black_oo_castle: bool,
    white_ooo_castle: bool,
    black_ooo_castle: bool,
}

pub struct BitBoards {
    pub bb_white_pawn: u64,
    pub bb_white_rook: u64,
    pub bb_white_bishop: u64,
    pub bb_white_knight: u64,
    pub bb_white_queen: u64,
    pub bb_white_king: u64,
    pub bb_black_pawn: u64,
    pub bb_black_rook: u64,
    pub bb_black_bishop: u64,
    pub bb_black_knight: u64,
    pub bb_black_queen: u64,
    pub bb_black_king: u64
}

impl BitBoards {
    pub fn clean() -> Self {
        return Self { 
            bb_white_pawn: 0, 
            bb_white_rook: 0, 
            bb_white_bishop: 0, 
            bb_white_knight: 0, 
            bb_white_queen: 0, 
            bb_white_king: 0, 
            bb_black_pawn: 0, 
            bb_black_rook: 0, 
            bb_black_bishop: 0, 
            bb_black_knight: 0, 
            bb_black_queen: 0, 
            bb_black_king: 0 
        }
    }
}