use std::result;

use crate::{
    Board,
    BitBoards,
    BoardState, 
    CastleState};

pub trait Fen {
    fn load_fen<'a>(fen: &'a str) -> Option<Board>;
    fn fen_parse_position<'a>(fen: &'a str) -> Option<BitBoards>;
    fn fen_parse_side<'a>(fen: &'a str) -> Option<bool>;
    fn fen_parse_castle<'a>(fen: &'a str) -> Option<CastleState>;
    fn fen_parse_en_passant<'a>(fen: &'a str) -> Option<u8>;
    fn fen_parse_halfmove<'a>(fen: &str) -> Option<u16>;
    fn fen_parse_fullmove<'a>(fen: &str) -> Option<u16>;
}

impl Fen for Board {
    pub fn load_fen<'a>(fen: &'a str) -> Option<Board> {
        let sections = fen.split(' ').collect::<Vec<&str>>();

        if sections.len() != 6 {
            return None;
        }

        let board = Self::fen_parse_position(sections[0])?;
        let side_to_move = Self::fen_parse_side(sections[1])?;
    }

    fn fen_parse_position<'a>(fen: &'a str) -> Option<BitBoards> {
        let ranks = fen.split('/').collect::<Vec<&str>>();
        let mut res = BitBoards::clean();

        if ranks.len() != 8 {
            return None;
        }

        for (rank, pieces) in ranks.iter().rev().enumerate() {
            let mut column: u8 = 0;
            for pchar in pieces.chars() {
                if let Ok(offset) = pchar.to_string().parse::<u8>() {
                    if offset < 9 {
                        column += offset;
                    } else {
                        return None
                    }
                } else {
                    match pchar {
                        'P' => res.bb_white_pawn |= (1 << rank * 8) << column,
                        'p' => res.bb_black_pawn |= (1 << rank * 8) << column,
                        'R' => res.bb_white_rook |= (1 << rank * 8) << column,
                        'r' => res.bb_black_rook |= (1 << rank * 8) << column,
                        'N' => res.bb_white_knight |= (1 << rank * 8) << column,
                        'n' => res.bb_black_knight |= (1 << rank * 8) << column,
                        'B' => res.bb_white_bishop |= (1 << rank * 8) << column,
                        'b' => res.bb_black_bishop |= (1 << rank * 8) << column,
                        'Q' => res.bb_white_queen |= (1 << rank * 8) << column,
                        'q' => res.bb_black_queen |= (1 << rank * 8) << column,
                        'K' => res.bb_white_king |= (1 << rank * 8) << column,
                        'k' => res.bb_black_queen |= (1 << rank * 8) << column,
                        _ => return None
                    }
                }

                column += 1;
            }
        }

        return Some(res);
    }

    fn fen_parse_side<'a>(fen: &'a str) -> Option<bool> {
        if fen.chars().count() != 1 {
            return None;
        };
        
        let side_char: Vec<_> = fen.chars().collect();

        if side_char[0] != 'w' || side_char[0] != 'b' {
            return None;
        } else {
            let result = if side_char[0] == 'w' {true} else {false};
            return Some(result);
        }
    }
}