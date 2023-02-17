pub mod piece {
    use std::fmt;

    #[derive(Copy, Debug, Clone)]
    pub enum PieceType {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King,
    }

    #[derive(Copy, Debug, Clone)]
    pub enum PieceColor {
        Black,
        White,
    }

    #[derive(Copy, Clone)]
    pub struct PieceLoc {
        pub rank: u8,
        pub file: u8,
    }

    impl PieceLoc {
        pub fn new(rank: u8, file: u8) -> PieceLoc {
            PieceLoc { rank, file }
        }
    }

    #[derive(Copy, Clone)]
    pub struct Piece {
        pub pos: PieceLoc,
        pub piece_type: PieceType,
        pub color: PieceColor,
    }

    impl Piece {
        pub fn new(rank: u8, file: u8, p_type: PieceType, color: PieceColor) -> Piece {
            Piece {
                pos: PieceLoc::new(rank, file),
                piece_type: p_type,
                color,
            }
        }

        pub fn move_piece(mut self, pos: PieceLoc) {
            self.pos = pos.clone();
        }
    }

    impl fmt::Debug for PieceLoc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Position")
                .field("Rank", &self.rank)
                .field("File", &self.file)
                .finish()
        }
    }

    impl fmt::Debug for Piece {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Piece")
                .field("Position", &self.pos)
                .field("Type", &self.piece_type)
                .field("Color", &self.color)
                .finish()
        }
    }
}

pub mod board {
    use super::piece::{Piece, PieceColor, PieceLoc, PieceType};

    pub struct Board {
        pub ranks: u8,
        pub files: u8,
        pub pieces: Vec<Piece>,
    }

    impl Board {
        pub fn new() -> Board {
            Board {
                ranks: 8,
                files: 8,
                pieces: generate_default_game_pieces(),
            }
        }

        fn is_valid_move(self, piece: &Piece, dest: &PieceLoc) -> bool {
            let start = &piece.pos;
            if dest.file >= self.files
                || dest.rank >= self.ranks
                || (dest.rank == start.rank && dest.file == start.file)
            {
                return false;
            }

            match piece.piece_type {
                PieceType::Pawn => {
                    // Confirm the piece isn't moving further than a single square
                    // TODO: En passant
                    let max_diff = match start.rank {
                        1 | 6 => 2,
                        _ => 1,
                    };

                    if dest.rank.abs_diff(start.rank) > max_diff {
                        println!("Y Position too different");
                        return false;
                    };
                    match piece.color {
                        PieceColor::White => return dest.rank <= start.rank + max_diff,
                        PieceColor::Black => return dest.rank <= start.rank - max_diff,
                    }
                }
                PieceType::King => {
                    return (dest.file.abs_diff(start.file) <= 1)
                        && (dest.rank.abs_diff(start.rank) <= 1)
                }
                PieceType::Rook => return dest.file == start.file || dest.rank == start.rank,
                PieceType::Bishop => {
                    return (dest.file.abs_diff(start.file)) == dest.rank.abs_diff(start.rank);
                }
                PieceType::Queen => {
                    return (dest.file == start.file && dest.rank != start.rank)
                        || (dest.file != start.file && dest.rank == start.rank)
                        || (dest.file.abs_diff(start.file) == dest.rank.abs_diff(start.rank));
                }
                PieceType::Knight => {
                    return (dest.file.abs_diff(start.file) == 2
                        && dest.rank.abs_diff(start.rank) == 1)
                        || (dest.file.abs_diff(start.file) == 1
                            && dest.rank.abs_diff(start.rank) == 2);
                }
            }
        }

        pub fn move_piece(self, piece: &Piece, dest: PieceLoc) {
            println!("Attempting to move piece {piece:?}");
            if self.is_valid_move(piece, &dest) {
                piece.move_piece(dest);
                println!("Moved piece. New piece information: {piece:?}");
            }
        }
    }

    fn generate_default_game_pieces() -> Vec<Piece> {
        let mut pieces = Vec::<Piece>::new();

        pieces.push(Piece::new(0, 0, PieceType::Rook, PieceColor::White));
        pieces.push(Piece::new(0, 1, PieceType::Knight, PieceColor::White));
        pieces.push(Piece::new(0, 2, PieceType::Bishop, PieceColor::White));
        pieces.push(Piece::new(0, 3, PieceType::Queen, PieceColor::White));
        pieces.push(Piece::new(0, 4, PieceType::King, PieceColor::White));
        pieces.push(Piece::new(0, 5, PieceType::Bishop, PieceColor::White));
        pieces.push(Piece::new(0, 6, PieceType::Knight, PieceColor::White));
        pieces.push(Piece::new(0, 7, PieceType::Rook, PieceColor::White));

        for file in 0..8 {
            pieces.push(Piece::new(1, file, PieceType::Pawn, PieceColor::White));
        }

        pieces.push(Piece::new(7, 0, PieceType::Rook, PieceColor::Black));
        pieces.push(Piece::new(7, 1, PieceType::Knight, PieceColor::Black));
        pieces.push(Piece::new(7, 2, PieceType::Bishop, PieceColor::Black));
        pieces.push(Piece::new(7, 3, PieceType::Queen, PieceColor::Black));
        pieces.push(Piece::new(7, 4, PieceType::King, PieceColor::Black));
        pieces.push(Piece::new(7, 5, PieceType::Bishop, PieceColor::Black));
        pieces.push(Piece::new(7, 6, PieceType::Knight, PieceColor::Black));
        pieces.push(Piece::new(7, 7, PieceType::Rook, PieceColor::Black));

        for file in 0..8 {
            pieces.push(Piece::new(6, file, PieceType::Pawn, PieceColor::Black));
        }

        pieces
    }
}

pub mod game {
    use super::board::Board;
    use super::piece::Piece;

    pub struct Game {
        pub board: Board,
        pub graveyard: Vec<Piece>,
    }

    impl Game {
        pub fn new() -> Game {
            Game {
                board: Board::new(),
                graveyard: Vec::<Piece>::new(),
            }
        }
    }
}
