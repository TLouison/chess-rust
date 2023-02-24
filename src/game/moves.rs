use crate::game::board;
use crate::game::piece::{piece_info::PieceLoc, Piece};

pub mod move_checker;

#[derive(PartialEq)]
pub enum CaptureResult {
    Normal,
    EnPassant,
    None,
}

#[derive(Clone, Debug)]
pub struct Move {
    pub piece: Piece,
    pub start_pos: PieceLoc,
    pub end_pos: PieceLoc,
}

impl Move {
    pub fn new(piece: Piece, start: PieceLoc, dest: PieceLoc) -> Move {
        Move {
            piece: piece.clone(),
            start_pos: start.clone(),
            end_pos: dest.clone(),
        }
    }

    pub fn get_move_display(&self) -> String {
        format!(
            "{}{}{}",
            board::board_display::get_piece_display(&self.piece, true),
            board::board_display::convert_rank_numeric_to_alpha(self.end_pos.file)
                .expect("Somehow converted a file > 7"),
            self.end_pos.rank + 1
        )
    }
}
