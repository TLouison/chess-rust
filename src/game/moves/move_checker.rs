use crate::game::board::Board;
use crate::game::piece::{
    piece_info::{PieceColor, PieceLoc, PieceType},
    Piece,
};
use core::fmt;

use super::CaptureResult;

#[derive(Debug)]
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
) -> Result<CaptureResult, MoveError> {
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
    let mut capturing_move = CaptureResult::None;
    if let Some(existing_piece) = board.get_piece_at_location(*dest) {
        if existing_piece.color == piece.color {
            return Err(MoveError::OccupiedBySameColor);
        } else {
            capturing_move = CaptureResult::Normal;
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

            // Special case: check for en passant conditions
            if let Some(last_move) = board.get_previous_move() {
                if last_move.piece.piece_type == PieceType::Pawn {
                    match last_move.piece.color {
                        PieceColor::White => {
                            if last_move.start_pos.rank == 1 && last_move.end_pos.rank == 3 {
                                if dest.rank == 2 && dest.file == last_move.end_pos.file {
                                    capturing_move = CaptureResult::EnPassant;
                                }
                            }
                        }
                        PieceColor::Black => {
                            if last_move.start_pos.rank == 6 && last_move.end_pos.rank == 4 {
                                if dest.rank == 5 && dest.file == last_move.end_pos.file {
                                    capturing_move = CaptureResult::EnPassant;
                                }
                            }
                        }
                    }
                }
            }

            if capturing_move != CaptureResult::None {
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
                Err(MoveError::RankDifferenceGreater)
            } else {
                Ok(capturing_move)
            }
        }
        PieceType::Rook => {
            if is_cardinal_move(start, dest) {
                Ok(capturing_move)
            } else {
                Err(MoveError::RookMustMoveCardinal)
            }
        }
        PieceType::Bishop => {
            if is_diagonal_move(start, dest) {
                Ok(capturing_move)
            } else {
                Err(MoveError::BishopMustMoveDiagonal)
            }
        }
        PieceType::Queen => {
            if is_diagonal_move(start, dest) || is_cardinal_move(start, dest) {
                Ok(capturing_move)
            } else {
                Err(MoveError::MoveNotStraightLine)
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
    fn test_minimal_diagonal_move_all_directions_return_true() {
        assert_eq!(
            true,
            is_diagonal_move(&PieceLoc::new(1, 1), &PieceLoc::new(2, 2))
        );
        assert_eq!(
            true,
            is_diagonal_move(&PieceLoc::new(1, 1), &PieceLoc::new(2, 0))
        );
        assert_eq!(
            true,
            is_diagonal_move(&PieceLoc::new(1, 1), &PieceLoc::new(0, 2))
        );
        assert_eq!(
            true,
            is_diagonal_move(&PieceLoc::new(1, 1), &PieceLoc::new(0, 0))
        );
    }

    #[test]
    fn test_diagonal_move_all_cardinal_directions_return_false() {
        assert_eq!(
            false,
            is_diagonal_move(&PieceLoc::new(1, 1), &PieceLoc::new(2, 1))
        );
        assert_eq!(
            false,
            is_diagonal_move(&PieceLoc::new(1, 1), &PieceLoc::new(1, 2))
        );
        assert_eq!(
            false,
            is_diagonal_move(&PieceLoc::new(1, 1), &PieceLoc::new(0, 1))
        );
        assert_eq!(
            false,
            is_diagonal_move(&PieceLoc::new(1, 1), &PieceLoc::new(1, 0))
        );
    }

    #[test]
    fn test_diagonal_move_bad_returns_false() {
        // Knight move
        assert_eq!(
            false,
            is_diagonal_move(&PieceLoc::new(1, 1), &PieceLoc::new(3, 2))
        );
        // Completely bad move
        assert_eq!(
            false,
            is_diagonal_move(&PieceLoc::new(1, 1), &PieceLoc::new(3, 7))
        );
    }

    #[test]
    fn test_minimal_cardinal_move_all_directions_return_true() {
        assert_eq!(
            true,
            is_cardinal_move(&PieceLoc::new(1, 1), &PieceLoc::new(2, 1))
        );
        assert_eq!(
            true,
            is_cardinal_move(&PieceLoc::new(1, 1), &PieceLoc::new(1, 2))
        );
        assert_eq!(
            true,
            is_cardinal_move(&PieceLoc::new(1, 1), &PieceLoc::new(0, 1))
        );
        assert_eq!(
            true,
            is_cardinal_move(&PieceLoc::new(1, 1), &PieceLoc::new(1, 0))
        );
    }

    #[test]
    fn test_cardinal_move_all_diagonal_directions_return_false() {
        assert_eq!(
            false,
            is_cardinal_move(&PieceLoc::new(1, 1), &PieceLoc::new(2, 2))
        );
        assert_eq!(
            false,
            is_cardinal_move(&PieceLoc::new(1, 1), &PieceLoc::new(0, 0))
        );
        assert_eq!(
            false,
            is_cardinal_move(&PieceLoc::new(1, 1), &PieceLoc::new(2, 0))
        );
        assert_eq!(
            false,
            is_cardinal_move(&PieceLoc::new(1, 1), &PieceLoc::new(0, 2))
        );
    }

    #[cfg(test)]
    mod pawn_tests {
        use crate::game::board::Board;
        use crate::game::moves::Move;
        use crate::game::piece::piece_info::{PieceLoc, PieceType};

        fn setup_e4() -> Board {
            let mut board = Board::new();
            board.board[28] = board.board[12];
            board.board[12] = None;
            board
        }

        fn setup_e4_e5() -> Board {
            let mut board = setup_e4();
            board.board[36] = board.board[52];
            board.board[52] = None;
            board
        }

        fn setup_e4_d5() -> Board {
            let mut board = setup_e4();
            board.board[35] = board.board[51];
            board.board[51] = None;
            board
        }

        #[test]
        fn test_valid_move_pawn_forward_one() {
            let board = Board::new();
            let piece = board.board[12].unwrap(); // E2

            assert_eq!(piece.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 1, file: 5 };
            let end_pos = PieceLoc { rank: 2, file: 5 };

            let verdict = super::is_valid_move(&board, &piece, &start_pos, &end_pos);

            assert_eq!(true, verdict.is_ok());
        }

        #[test]
        fn test_valid_move_pawn_forward_two() {
            let board = Board::new();
            let piece = board.board[12].unwrap(); // E2

            assert_eq!(piece.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 1, file: 5 };
            let end_pos = PieceLoc { rank: 3, file: 5 };

            let verdict = super::is_valid_move(&board, &piece, &start_pos, &end_pos);

            assert_eq!(true, verdict.is_ok());
        }

        #[test]
        fn test_valid_move_pawn_forward_one_after_already_moving() {
            let board = setup_e4();
            let piece = board.board[28].unwrap(); // E4

            assert_eq!(piece.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 3, file: 5 };
            let end_pos = PieceLoc { rank: 4, file: 5 };

            let verdict = super::is_valid_move(&board, &piece, &start_pos, &end_pos);

            assert_eq!(true, verdict.is_ok());
        }

        #[test]
        fn test_invalid_move_pawn_forward_two_after_already_moving() {
            let board = setup_e4();
            let piece = board.board[28].unwrap(); // E4

            assert_eq!(piece.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 3, file: 4 };
            let end_pos = PieceLoc { rank: 5, file: 4 };

            let verdict = super::is_valid_move(&board, &piece, &start_pos, &end_pos);

            assert_eq!(false, verdict.is_ok());
        }

        #[test]
        fn test_invalid_move_pawn_forward_piece_blocking() {
            let board = setup_e4_e5();

            let piece = board.board[28].unwrap(); // E4
            assert_eq!(piece.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 3, file: 4 };
            let end_pos = PieceLoc { rank: 4, file: 4 };

            let verdict = super::is_valid_move(&board, &piece, &start_pos, &end_pos);

            assert_eq!(false, verdict.is_ok());
        }

        #[test]
        fn test_invalid_move_pawn_backward() {
            let board = setup_e4();
            let piece = board.board[28].unwrap(); // E4

            assert_eq!(piece.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 3, file: 4 };
            let end_pos = PieceLoc { rank: 2, file: 4 };

            let verdict = super::is_valid_move(&board, &piece, &start_pos, &end_pos);

            assert_eq!(false, verdict.is_ok());
        }

        #[test]
        fn test_valid_move_capture_diagonally() {
            let board = setup_e4_d5();
            let piece = board.board[28].unwrap(); // E4

            assert_eq!(piece.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 3, file: 4 };
            let end_pos = PieceLoc { rank: 4, file: 3 };

            let verdict = super::is_valid_move(&board, &piece, &start_pos, &end_pos);

            assert_eq!(true, verdict.is_ok());
            assert_eq!(true, verdict.unwrap()); // Capture returns bool = true
        }

        #[test]
        fn test_valid_move_en_passant() {
            let mut board = setup_e4_d5();
            let white_e_pawn = board.board[28].unwrap();
            let black_f_pawn = board.board[53].unwrap();
            board = board.move_piece(Move {
                piece: white_e_pawn,
                start_pos: PieceLoc { rank: 3, file: 4 },
                end_pos: PieceLoc { rank: 4, file: 4 },
            });
            board = board.move_piece(Move {
                piece: black_f_pawn,
                start_pos: PieceLoc { rank: 6, file: 5 },
                end_pos: PieceLoc { rank: 4, file: 5 },
            });

            // Can only en passant pawns
            assert_eq!(white_e_pawn.piece_type, PieceType::Pawn);
            assert_eq!(black_f_pawn.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 4, file: 4 };
            let end_pos = PieceLoc { rank: 5, file: 5 };

            let verdict = super::is_valid_move(&board, &white_e_pawn, &start_pos, &end_pos);
            assert_eq!(true, verdict.is_ok());
            assert_eq!(true, verdict.unwrap());
        }

        #[test]
        fn test_invalid_move_en_passant_after_1_space_move() {
            let mut board = setup_e4_d5();
            let white_e_pawn = board.board[28].unwrap();
            let black_f_pawn = board.board[53].unwrap();
            let other_white_piece = board.board[8].unwrap();
            board = board.move_piece(Move {
                piece: white_e_pawn,
                start_pos: PieceLoc { rank: 3, file: 4 },
                end_pos: PieceLoc { rank: 4, file: 4 },
            });
            board = board.move_piece(Move {
                piece: black_f_pawn,
                start_pos: PieceLoc { rank: 6, file: 5 },
                end_pos: PieceLoc { rank: 5, file: 5 },
            });
            board = board.move_piece(Move {
                piece: other_white_piece,
                start_pos: PieceLoc { rank: 1, file: 0 },
                end_pos: PieceLoc { rank: 2, file: 0 },
            });
            board = board.move_piece(Move {
                piece: black_f_pawn,
                start_pos: PieceLoc { rank: 5, file: 5 },
                end_pos: PieceLoc { rank: 4, file: 5 },
            });

            // Can only en passant pawns
            assert_eq!(white_e_pawn.piece_type, PieceType::Pawn);
            assert_eq!(black_f_pawn.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 4, file: 4 };
            let end_pos = PieceLoc { rank: 5, file: 5 };

            let verdict = super::is_valid_move(&board, &white_e_pawn, &start_pos, &end_pos);
            assert_eq!(false, verdict.is_ok());
        }
    }
}
