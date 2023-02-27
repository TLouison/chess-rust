use crate::game::board::{self, Board};
use crate::game::piece::{piece_info::PieceLoc, Piece};

use self::move_checker::MoveError;

pub mod move_checker;

#[derive(Clone, PartialEq, Debug)]
pub enum MoveType {
    Normal,
    EnPassant,
    Castling,
}

pub struct MoveResult {
    move_type: MoveType,
    capturing: bool,
}

#[derive(Clone, Debug)]
pub struct Move {
    pub piece: Piece,
    pub start_pos: PieceLoc,
    pub end_pos: PieceLoc,
    pub move_type: MoveType,
    pub capturing: bool,
}

impl Move {
    // Creates a move, checking first that the move is valid on the given board. This ensures
    // we cannot ever create an invalid move.
    pub fn new(
        board: &Board,
        piece: &Piece,
        start: &PieceLoc,
        dest: &PieceLoc,
    ) -> Result<Move, MoveError> {
        let move_result = move_checker::is_valid_move(board, piece, start, dest);
        match move_result {
            Ok(result) => Ok(Move {
                piece: piece.clone(),
                start_pos: start.clone(),
                end_pos: dest.clone(),
                move_type: result.move_type.clone(),
                capturing: result.capturing,
            }),
            Err(e) => Err(e),
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
