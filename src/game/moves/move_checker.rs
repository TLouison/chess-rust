use crate::game::board::Board;
use crate::game::piece::{
    piece_info::{PieceColor, PieceLoc, PieceType},
    Piece,
};
use core::fmt;

use super::Move;

#[derive(Clone, PartialEq, Debug)]
pub enum MoveType {
    Normal,
    EnPassant,
    Castling,
}

#[derive(Debug)]
pub struct MoveResult {
    pub move_type: MoveType,
    pub capturing: bool,
}

#[derive(Debug, PartialEq)]
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
    PawnEnPassantNotValid,
    KnightInvalidMove,
    RookMustMoveCardinal,
    BishopMustMoveDiagonal,
    NoRookToCastleWith,
    CannotCastleWithMovedRook,
    CannotCastleWithMovedKing,
    CannotCastleThroughPiece,
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
                    MoveError::PawnEnPassantNotValid => output = "Conditions not met to perform en passant",
                    MoveError::KnightInvalidMove => output = "Knights may only move two squares in one cardinal direction, and one square in a perpendicular direction.",
                    MoveError::RookMustMoveCardinal => output = "Rooks may only move horizontally or vertically.",
                    MoveError::BishopMustMoveDiagonal => output = "Bishops may only move diagonally.",
                    MoveError::NoRookToCastleWith => output = "There is no valid rook to castle with on that side.",
                    MoveError::CannotCastleWithMovedRook => output = "You cannot castle with a rook that has previously moved.",
                    MoveError::CannotCastleWithMovedKing => output = "You cannot castle with a king that has previously moved.",
                    MoveError::CannotCastleThroughPiece => output = "You cannot castle with a piece between the king and rook.",
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
    (dest.file.abs_diff(start.file) == 2 && dest.rank.abs_diff(start.rank) == 1)
        || (dest.file.abs_diff(start.file) == 1 && dest.rank.abs_diff(start.rank) == 2)
}

fn can_en_passant(
    board: &Board,
    piece: &Piece,
    start: &PieceLoc,
    dest: &PieceLoc,
) -> Result<MoveResult, MoveError> {
    // Special case: check for en passant conditions
    let (required_start_rank, valid_destination_rank) = match piece.color {
        PieceColor::White => (4, 5),
        PieceColor::Black => (2, 1),
    };

    println!("{:?}", required_start_rank);

    // If moving pawn is in correct spot
    if start.rank == required_start_rank {
        println!("In correct spot");
        if let Some(last_move) = board.get_previous_move() {
            // Previous move was a pawn move
            if last_move.piece.piece_type == PieceType::Pawn {
                println!("Last move was pawn");
                // Previous pawn move was a two-square move
                if last_move.start_pos.rank.abs_diff(last_move.end_pos.rank) == 2 {
                    // Attempting a capture to the square behind the white pawn that moved two
                    println!("Last move was 2 spaces");
                    if dest.rank == valid_destination_rank && dest.file == last_move.end_pos.file {
                        return Ok(MoveResult {
                            move_type: MoveType::EnPassant,
                            capturing: true,
                        });
                    }
                }
            }
        }
    }
    return Err(MoveError::PawnEnPassantNotValid);
}

// fn valid_destinations(board: &Board, piece: &Piece, current_loc: &PieceLoc) -> Vec<PieceLoc> {
//     let valid_dests = Vec::new();
//     match piece.piece_type {
//         PieceType::Pawn => {
//             let direction: i8 = match piece.color {
//                 PieceColor::White => 1,
//                 PieceColor::Black => -1,
//             }
//             // Move forward 1 rank
//             let one_rank_forward = (current_loc.rank as i8 + (1 * direction)) as u8;
//             if let None =
//                 board.get_piece_at_location(PieceLoc::new(one_rank_forward, current_loc.file))
//             {
//                 valid_dests.push(PieceLoc::new(one_rank_forward, current_loc.file));
//             }
//             // Move forward 2 ranks if first move
//             let two_rank_forward = (current_loc.rank as i8 + (2 * direction)) as u8;
//             if piece.has_moved {
//                 if let None = board
//                     .get_piece_at_location(PieceLoc::new(two_rank_forward, current_loc.file))
//                 {
//                     valid_dests.push(PieceLoc::new(two_rank_forward, current_loc.file));
//                 }
//             }

//         }
//     }
//     valid_dests;
// }

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
) -> Result<MoveResult, MoveError> {
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

    // Check if there is a piece at the target destination, making this a capturing move
    let mut capturing = false;
    let mut move_type = MoveType::Normal;
    if let Some(existing_piece) = board.get_piece_at_location(*dest) {
        if existing_piece.color == piece.color {
            return Err(MoveError::OccupiedBySameColor);
        } else {
            capturing = true;
        }
    }

    match piece.piece_type {
        PieceType::Pawn => {
            let max_diff = match piece.has_moved {
                false => 2,
                true => 1,
            };

            let direction: i8 = match piece.color {
                PieceColor::White => 1,
                PieceColor::Black => -1,
            };

            // Confirm pawn cannot move 2 squares unless on the starting position
            if dest.rank.abs_diff(start.rank) > max_diff {
                return Err(MoveError::RankDifferenceGreater);
            };

            // Confirm pawn is moving in correct direction
            if (dest.rank as i8 * direction) <= (start.rank as i8 * direction) {
                return Err(MoveError::PawnMustMoveForward);
            }

            // Special case: check for en passant conditions
            let en_passant_result = can_en_passant(board, piece, start, dest);
            if en_passant_result.is_ok() {
                capturing = true;
                move_type = MoveType::EnPassant;
            }

            if capturing {
                // Pawns can only capture diagonally adjacent pieces
                if dest.file.abs_diff(start.file) != 1 || dest.rank.abs_diff(start.rank) != 1 {
                    return Err(MoveError::PawnMustCaptureDiagonal);
                }
            } else {
                if start.file != dest.file {
                    return Err(MoveError::PawnMustMoveForward);
                }
            }

            Ok(MoveResult {
                move_type,
                capturing,
            })
        }
        PieceType::King => {
            // SPECIAL MOVE: Castling
            if dest.rank == start.rank && dest.file.abs_diff(start.file) == 2 {
                // King cannot have moved for castling to be valid
                if piece.has_moved {
                    return Err(MoveError::CannotCastleWithMovedKing);
                }

                let castling_rook;
                if dest.file < start.file {
                    // Castling queenside
                    castling_rook = board.get_piece_at_location(PieceLoc::new(dest.rank, 0));
                } else {
                    // Castling kingside
                    castling_rook = board.get_piece_at_location(PieceLoc::new(dest.rank, 7));
                }

                if let Some(rook) = castling_rook {
                    if !rook.has_moved {
                        move_type = MoveType::Castling;
                        Ok(MoveResult {
                            move_type,
                            capturing,
                        })
                    } else {
                        Err(MoveError::CannotCastleWithMovedRook)
                    }
                } else {
                    Err(MoveError::NoRookToCastleWith)
                }
            } else if dest.file.abs_diff(start.file) > 1 {
                Err(MoveError::FileDifferenceGreater)
            } else if dest.rank.abs_diff(start.rank) > 1 {
                Err(MoveError::RankDifferenceGreater)
            } else {
                Ok(MoveResult {
                    move_type,
                    capturing,
                })
            }
        }
        PieceType::Rook => {
            if is_cardinal_move(start, dest) {
                Ok(MoveResult {
                    move_type,
                    capturing,
                })
            } else {
                Err(MoveError::RookMustMoveCardinal)
            }
        }
        PieceType::Bishop => {
            if is_diagonal_move(start, dest) {
                Ok(MoveResult {
                    move_type,
                    capturing,
                })
            } else {
                Err(MoveError::BishopMustMoveDiagonal)
            }
        }
        PieceType::Queen => {
            if is_diagonal_move(start, dest) || is_cardinal_move(start, dest) {
                Ok(MoveResult {
                    move_type,
                    capturing,
                })
            } else {
                Err(MoveError::MoveNotStraightLine)
            }
        }
        PieceType::Knight => {
            if is_knight_move(start, dest) {
                Ok(MoveResult {
                    move_type,
                    capturing,
                })
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
        use crate::game::moves::{move_checker::MoveType, Move};
        use crate::game::piece::piece_info::{PieceColor, PieceLoc, PieceType};

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
        fn test_valid_move_pawn_forward_one_white() {
            let board = Board::new();
            let piece = board.board[12].unwrap(); // E2

            assert_eq!(piece.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 1, file: 5 };
            let end_pos = PieceLoc { rank: 2, file: 5 };

            let verdict = super::is_valid_move(&board, &piece, &start_pos, &end_pos);
            assert_eq!(true, verdict.is_ok());

            let verdict = verdict.unwrap();
            assert_eq!(false, verdict.capturing);
            assert_eq!(MoveType::Normal, verdict.move_type);
        }

        #[test]
        fn test_valid_move_pawn_forward_one_black() {
            let mut board = Board::new();
            board.current_turn = PieceColor::Black;
            let piece = board.board[52].unwrap(); // E2

            assert_eq!(piece.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 6, file: 5 };
            let end_pos = PieceLoc { rank: 5, file: 5 };

            let verdict = super::is_valid_move(&board, &piece, &start_pos, &end_pos);
            println!("{:?}", verdict);
            assert_eq!(true, verdict.is_ok());

            let verdict = verdict.unwrap();
            assert_eq!(false, verdict.capturing);
            assert_eq!(MoveType::Normal, verdict.move_type);
        }

        #[test]
        fn test_valid_move_pawn_forward_two_white() {
            let board = Board::new();
            let piece = board.board[12].unwrap(); // E2

            assert_eq!(piece.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 1, file: 5 };
            let end_pos = PieceLoc { rank: 3, file: 5 };

            let verdict = super::is_valid_move(&board, &piece, &start_pos, &end_pos);
            println!("{:?}", verdict);

            assert_eq!(true, verdict.is_ok());

            let verdict = verdict.unwrap();
            assert_eq!(false, verdict.capturing);
            assert_eq!(MoveType::Normal, verdict.move_type);
        }

        #[test]
        fn test_valid_move_pawn_forward_two_black() {
            let mut board = Board::new();
            board.current_turn = PieceColor::Black;
            let piece = board.board[52].unwrap(); // E2

            assert_eq!(piece.piece_type, PieceType::Pawn);

            let start_pos = PieceLoc { rank: 6, file: 5 };
            let end_pos = PieceLoc { rank: 4, file: 5 };

            let verdict = super::is_valid_move(&board, &piece, &start_pos, &end_pos);
            println!("{:?}", verdict);

            assert_eq!(true, verdict.is_ok());

            let verdict = verdict.unwrap();
            assert_eq!(false, verdict.capturing);
            assert_eq!(MoveType::Normal, verdict.move_type);
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

            let verdict = verdict.unwrap();
            assert_eq!(false, verdict.capturing);
            assert_eq!(MoveType::Normal, verdict.move_type);
        }

        #[test]
        fn test_invalid_move_pawn_forward_two_after_already_moving() {
            let board = setup_e4();
            let mut piece = board.board[28].unwrap(); // E4
            piece.has_moved = true;

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

            let verdict = verdict.unwrap();
            assert_eq!(true, verdict.capturing);
            assert_eq!(MoveType::Normal, verdict.move_type);
        }

        #[test]
        fn test_valid_move_en_passant_white() {
            let mut board = setup_e4_d5();
            let white_e_pawn = board.board[28].unwrap();
            let black_f_pawn = board.board[53].unwrap();

            board = board.move_piece(Move {
                piece: white_e_pawn,
                start_pos: PieceLoc { rank: 3, file: 4 },
                end_pos: PieceLoc { rank: 4, file: 4 },
                move_type: MoveType::Normal,
                capturing: false,
            });
            board = board.move_piece(Move {
                piece: black_f_pawn,
                start_pos: PieceLoc { rank: 6, file: 5 },
                end_pos: PieceLoc { rank: 4, file: 5 },
                move_type: MoveType::Normal,
                capturing: false,
            });

            assert_eq!(board.move_list[0].piece.color, PieceColor::White);
            assert_eq!(board.move_list[1].piece.color, PieceColor::Black);

            // Can only en passant pawns
            assert_eq!(white_e_pawn.piece_type, PieceType::Pawn);
            assert_eq!(black_f_pawn.piece_type, PieceType::Pawn);

            // Confirming pawns are next to each other after moving
            assert!(board.board[36].is_some());
            assert!(board.board[37].is_some());

            let start_pos = PieceLoc { rank: 4, file: 4 };
            let end_pos = PieceLoc { rank: 5, file: 5 };

            let verdict = super::is_valid_move(&board, &white_e_pawn, &start_pos, &end_pos);
            assert_eq!(true, verdict.is_ok());

            let verdict = verdict.unwrap();
            assert_eq!(true, verdict.capturing);
            assert_eq!(MoveType::EnPassant, verdict.move_type);
        }

        #[test]
        fn test_valid_move_en_passant_black() {
            let mut board = setup_e4_d5();
            let white_e_pawn = board.board[28].unwrap();
            let black_f_pawn = board.board[53].unwrap();

            board = board.move_piece(Move {
                piece: white_e_pawn,
                start_pos: PieceLoc { rank: 3, file: 4 },
                end_pos: PieceLoc { rank: 4, file: 4 },
                move_type: MoveType::Normal,
                capturing: false,
            });
            board = board.move_piece(Move {
                piece: black_f_pawn,
                start_pos: PieceLoc { rank: 6, file: 5 },
                end_pos: PieceLoc { rank: 4, file: 5 },
                move_type: MoveType::Normal,
                capturing: false,
            });

            assert_eq!(board.move_list[0].piece.color, PieceColor::White);
            assert_eq!(board.move_list[1].piece.color, PieceColor::Black);

            // Can only en passant pawns
            assert_eq!(white_e_pawn.piece_type, PieceType::Pawn);
            assert_eq!(black_f_pawn.piece_type, PieceType::Pawn);

            // Confirming pawns are next to each other after moving
            assert!(board.board[36].is_some());
            assert!(board.board[37].is_some());

            let start_pos = PieceLoc { rank: 4, file: 4 };
            let end_pos = PieceLoc { rank: 5, file: 5 };

            let verdict = super::is_valid_move(&board, &white_e_pawn, &start_pos, &end_pos);
            assert_eq!(true, verdict.is_ok());

            let verdict = verdict.unwrap();
            assert_eq!(true, verdict.capturing);
            assert_eq!(MoveType::EnPassant, verdict.move_type);
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
                move_type: MoveType::Normal,
                capturing: false,
            });
            board = board.move_piece(Move {
                piece: black_f_pawn,
                start_pos: PieceLoc { rank: 6, file: 5 },
                end_pos: PieceLoc { rank: 5, file: 5 },
                move_type: MoveType::Normal,
                capturing: false,
            });
            board = board.move_piece(Move {
                piece: other_white_piece,
                start_pos: PieceLoc { rank: 1, file: 0 },
                end_pos: PieceLoc { rank: 2, file: 0 },
                move_type: MoveType::Normal,
                capturing: false,
            });
            board = board.move_piece(Move {
                piece: black_f_pawn,
                start_pos: PieceLoc { rank: 5, file: 5 },
                end_pos: PieceLoc { rank: 4, file: 5 },
                move_type: MoveType::Normal,
                capturing: false,
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

    #[cfg(test)]
    mod knight_tests {
        use crate::game::board::Board;
        use crate::game::moves::move_checker::{is_knight_move, MoveType};
        use crate::game::piece::piece_info::PieceLoc;

        fn setup_board_for_knight_capture() -> Board {
            let mut board = Board::new();

            board.board[36] = board.board[1];
            board.board[1] = None;

            board
        }

        #[test]
        fn test_valid_knight_move_all_directions() {
            // Up-right
            assert_eq!(
                true,
                is_knight_move(&PieceLoc::new(3, 3), &PieceLoc::new(5, 4))
            );
            // Right-up
            assert_eq!(
                true,
                is_knight_move(&PieceLoc::new(3, 3), &PieceLoc::new(4, 5))
            );
            // Right-down
            assert_eq!(
                true,
                is_knight_move(&PieceLoc::new(3, 3), &PieceLoc::new(2, 5))
            );
            // Down-right
            assert_eq!(
                true,
                is_knight_move(&PieceLoc::new(3, 3), &PieceLoc::new(1, 4))
            );
            // Down-left
            assert_eq!(
                true,
                is_knight_move(&PieceLoc::new(3, 3), &PieceLoc::new(1, 2))
            );
            // Left-down
            assert_eq!(
                true,
                is_knight_move(&PieceLoc::new(3, 3), &PieceLoc::new(2, 1))
            );
            // Left-up
            assert_eq!(
                true,
                is_knight_move(&PieceLoc::new(3, 3), &PieceLoc::new(4, 1))
            );
            // Up-left
            assert_eq!(
                true,
                is_knight_move(&PieceLoc::new(3, 3), &PieceLoc::new(5, 2))
            );
        }

        #[test]
        fn test_invalid_knight_moves() {
            // Vertical
            assert_eq!(
                false,
                is_knight_move(&PieceLoc::new(3, 3), &PieceLoc::new(4, 3))
            );
            // Horizontal
            assert_eq!(
                false,
                is_knight_move(&PieceLoc::new(3, 3), &PieceLoc::new(3, 4))
            );
            // Diagonal
            assert_eq!(
                false,
                is_knight_move(&PieceLoc::new(3, 3), &PieceLoc::new(4, 4))
            );
        }

        #[test]
        fn test_knight_capture() {
            let board = setup_board_for_knight_capture();
            let piece = board.board[36].unwrap();

            let verdict =
                super::is_valid_move(&board, &piece, &PieceLoc::new(4, 4), &PieceLoc::new(6, 5));
            assert_eq!(true, verdict.is_ok());

            let verdict = verdict.unwrap();
            assert_eq!(true, verdict.capturing);
            assert_eq!(MoveType::Normal, verdict.move_type);
        }
    }

    #[cfg(test)]
    mod king_tests {
        use crate::game::board::Board;
        use crate::game::moves::move_checker::{MoveError, MoveType};
        use crate::game::piece::{piece_info::PieceLoc, Piece};

        fn setup_back_rank() -> Board {
            let mut board = Board::new();
            board.board[1] = None;
            board.board[2] = None;
            board.board[3] = None;
            board.board[5] = None;
            board.board[6] = None;
            board
        }

        fn setup_back_rank_invalid() -> Board {
            let mut board = Board::new();
            board.board[1] = None;
            board.board[3] = None;
            board.board[6] = None;
            board
        }

        #[test]
        fn test_castle_kingside_is_valid() {
            let board = setup_back_rank();
            let white_king = board.board[4].unwrap();

            let verdict = super::is_valid_move(
                &board,
                &white_king,
                &PieceLoc::new(0, 4),
                &PieceLoc::new(0, 6),
            );

            println!("{:?}", verdict);

            assert_eq!(true, verdict.is_ok());

            let verdict = verdict.unwrap();
            assert_eq!(MoveType::Castling, verdict.move_type);
            assert_eq!(false, verdict.capturing);
        }

        #[test]
        fn test_castle_queenside_is_valid() {
            let board = setup_back_rank();
            let white_king = board.board[4].unwrap();

            let verdict = super::is_valid_move(
                &board,
                &white_king,
                &PieceLoc::new(0, 4),
                &PieceLoc::new(0, 2),
            );

            assert_eq!(true, verdict.is_ok());

            let verdict = verdict.unwrap();
            assert_eq!(MoveType::Castling, verdict.move_type);
            assert_eq!(false, verdict.capturing);
        }

        #[test]
        fn test_castle_through_piece_is_invalid() {
            let board = setup_back_rank_invalid();
            let white_king = board.board[4].unwrap();

            let verdict = super::is_valid_move(
                &board,
                &white_king,
                &PieceLoc::new(0, 4),
                &PieceLoc::new(0, 2),
            );

            assert_eq!(false, verdict.is_ok());
            assert_eq!(
                MoveError::CannotCastleThroughPiece,
                verdict.expect_err("Piece in-between king and rook, this is invalid.")
            );
        }

        #[test]
        fn test_castle_moved_rook_is_invalid() {
            let mut board = setup_back_rank();
            let white_king = board.board[4].unwrap();
            let white_rook = board.board[0].unwrap();

            board.board[0] = Some(Piece {
                has_moved: true,
                ..white_rook
            });

            let verdict = super::is_valid_move(
                &board,
                &white_king,
                &PieceLoc::new(0, 4),
                &PieceLoc::new(0, 2),
            );

            assert_eq!(false, verdict.is_ok());
            assert_eq!(
                MoveError::CannotCastleWithMovedRook,
                verdict.expect_err("Queenside rook moved, this is invalid.")
            );
        }

        #[test]
        fn test_castle_moved_king_is_invalid() {
            let mut board = setup_back_rank();
            let white_king = board.board[4].unwrap();

            board.board[4] = Some(Piece {
                has_moved: true,
                ..white_king
            });

            let white_king = board.board[4].unwrap();

            let verdict = super::is_valid_move(
                &board,
                &white_king,
                &PieceLoc::new(0, 4),
                &PieceLoc::new(0, 2),
            );

            assert_eq!(false, verdict.is_ok());
            assert_eq!(
                MoveError::CannotCastleWithMovedKing,
                verdict.expect_err("King has moved, this is invalid.")
            );
        }
    }
}
