use crate::game::board::Board;
use crate::game::piece::{
    piece_info::{PieceColor, PieceLoc, PieceType},
    Piece,
};
use core::fmt;

pub enum MoveError {
    WrongColorPiece,
    RankDifferenceGreater,
    FileDifferenceGreater,
    MoveOutOfBounds,
    MoveNotStraightLine,
    NoPositionChange,
    OccupiedBySameColor,
    PawnMustMoveForward,
    PawnMustCaptureDiagonal,
    KnightInvalidMove,
    RookMustMoveCardinal,
    BishopMustMoveDiagonal,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output;
        match *self {
                    MoveError::WrongColorPiece => output = "It is not your turn to move.",
                    MoveError::RankDifferenceGreater => output = "Piece attempted to move too many ranks at once.",
                    MoveError::FileDifferenceGreater => output = "Piece attempted to move too many files at once.",
                    MoveError::MoveOutOfBounds => output = "Piece attempted to move out of bounds.",
                    MoveError::MoveNotStraightLine => output = "Piece attempted to move to an invalid square.",
                    MoveError::NoPositionChange => output = "A piece cannot be moved to the square it already occupies.",
                    MoveError::OccupiedBySameColor => output = "A piece cannot be moved to a square that is occupied by a piece of the same color.",
                    MoveError::PawnMustMoveForward => output = "Pawns can only move forward.",
                    MoveError::PawnMustCaptureDiagonal => output = "Pawns cannot capture pieces directly in front of them.",
                    MoveError::KnightInvalidMove => output = "Knights may only move two squares in one cardinal direction, and one square in a perpendicular direction.",
                    MoveError::RookMustMoveCardinal => output = "Rooks may only move horizontally or vertically.",
                    MoveError::BishopMustMoveDiagonal => output = "Bishops may only move diagonally.",
                }
        write!(f, "Invalid Move: {}", output)
    }
}

fn is_diagonal_move(start: &PieceLoc, dest: &PieceLoc) -> bool {
    dest.file.abs_diff(start.file) == dest.rank.abs_diff(start.rank)
}

fn is_cardinal_move(start: &PieceLoc, dest: &PieceLoc) -> bool {
    dest.file == start.file || dest.rank == start.rank
}

fn is_knight_move(start: &PieceLoc, dest: &PieceLoc) -> bool {
    return (dest.file.abs_diff(start.file) == 2 && dest.rank.abs_diff(start.rank) == 1)
        || (dest.file.abs_diff(start.file) == 1 && dest.rank.abs_diff(start.rank) == 2);
}

/// Handles checking every type of piece to confirm that a proposed move is valid.
///
/// If the move is valid, it will return Ok(bool), where the bool indicates whether
/// the move captured another piece (true) or not (false).
///
/// If the move is invalid, it will return Err(MoveError), which can be a number of
/// possible errors that the move violates.
pub fn is_valid_move(
    board: &Board,
    piece: &Piece,
    start: &PieceLoc,
    dest: &PieceLoc,
) -> Result<bool, MoveError> {
    // Confirm the correct color piece is being moved depending on whose turn it is
    if board.current_turn != piece.color {
        return Err(MoveError::WrongColorPiece);
    }

    // Confirm the player made a move within the board's limits, and that
    // it could theoretically move a piece from it's starting square.
    if dest.file >= board.files || dest.rank >= board.ranks {
        return Err(MoveError::MoveOutOfBounds);
    }
    if dest.rank == start.rank && dest.file == start.file {
        return Err(MoveError::NoPositionChange);
    }

    // Check if there is a piece in the way, making this a capturing move
    let mut capturing_move = false;
    if let Some(existing_piece) = board.get_piece_at_location(*dest) {
        if existing_piece.color == piece.color {
            return Err(MoveError::OccupiedBySameColor);
        } else {
            capturing_move = true;
        }
    }

    match piece.piece_type {
        PieceType::Pawn => {
            let max_diff = match start.rank {
                1 | 6 => 2,
                _ => 1,
            };

            // Confirm pawn cannot move 2 squares unless on the starting position
            if dest.rank.abs_diff(start.rank) > max_diff {
                return Err(MoveError::RankDifferenceGreater);
            };

            // Confirm pawn is moving in correct direction
            match piece.color {
                PieceColor::White => {
                    if dest.rank <= start.rank {
                        return Err(MoveError::PawnMustMoveForward);
                    }
                    if dest.rank > start.rank + max_diff {
                        return Err(MoveError::RankDifferenceGreater);
                    }
                }
                PieceColor::Black => {
                    if dest.rank >= start.rank {
                        return Err(MoveError::PawnMustMoveForward);
                    }
                    if dest.rank < start.rank - max_diff {
                        return Err(MoveError::RankDifferenceGreater);
                    }
                }
            }

            if capturing_move {
                // Pawns can only capture diagonally adjacent pieces
                if dest.file.abs_diff(start.file) != 1 || dest.rank.abs_diff(start.rank) != 1 {
                    return Err(MoveError::PawnMustCaptureDiagonal);
                }
            }

            Ok(capturing_move)
        }
        PieceType::King => {
            if dest.file.abs_diff(start.file) > 1 {
                Err(MoveError::FileDifferenceGreater)
            } else if dest.rank.abs_diff(start.rank) > 1 {
                Err(MoveError::FileDifferenceGreater)
            } else {
                Ok(capturing_move)
            }
        }
        PieceType::Rook => {
            if !is_cardinal_move(start, dest) {
                Err(MoveError::RookMustMoveCardinal)
            } else {
                Ok(capturing_move)
            }
        }
        PieceType::Bishop => {
            if !is_diagonal_move(start, dest) {
                Err(MoveError::BishopMustMoveDiagonal)
            } else {
                Ok(capturing_move)
            }
        }
        PieceType::Queen => {
            if !is_diagonal_move(start, dest) && !is_cardinal_move(start, dest) {
                Err(MoveError::MoveNotStraightLine)
            } else {
                Ok(capturing_move)
            }
        }
        PieceType::Knight => {
            if is_knight_move(start, dest) {
                Ok(capturing_move)
            } else {
                Err(MoveError::KnightInvalidMove)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_move__happy_path() {
        let result = is_diagonal_move(PieceLoc { rank: 0, file: 0 });
        assert_eq!(result, 4);
    }
}
