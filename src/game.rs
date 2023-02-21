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

    #[derive(Copy, Clone, PartialEq)]
    pub struct PieceLoc {
        pub rank: u8,
        pub file: u8,
    }

    impl PieceLoc {
        pub fn new(rank: u8, file: u8) -> PieceLoc {
            PieceLoc { rank, file }
        }
    }

    pub struct PieceResult {
        pub piece: Piece,
        pub index: usize,
    }

    #[derive(Copy, Clone)]
    pub struct Piece {
        pub pos: PieceLoc,
        pub piece_type: PieceType,
        pub color: PieceColor,
        pub alive: bool,
    }

    impl Piece {
        pub fn new(rank: u8, file: u8, p_type: PieceType, color: PieceColor) -> Piece {
            Piece {
                pos: PieceLoc::new(rank, file),
                piece_type: p_type,
                color,
                alive: true,
            }
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
                .field("Alive", &self.alive)
                .finish()
        }
    }
}

pub mod board {
    use core::fmt;

    use super::piece::{Piece, PieceColor, PieceLoc, PieceResult, PieceType};

    #[derive(Clone, Debug)]
    pub struct Move {
        pub piece_idx: usize,
        pub start_pos: PieceLoc,
        pub end_pos: PieceLoc,
    }

    impl Move {
        pub fn new(piece_idx: usize, start: PieceLoc, dest: PieceLoc) -> Move {
            Move {
                piece_idx: piece_idx.clone(),
                start_pos: start.clone(),
                end_pos: dest.clone(),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Board {
        ranks: u8,
        files: u8,
        pub current_turn: PieceColor,
        pub move_list: Vec<Move>,
        pub pieces: Vec<Piece>,
        pub graveyard: Vec<Piece>,
    }

    impl Board {
        pub fn new() -> Board {
            Board {
                ranks: 8,
                files: 8,
                current_turn: PieceColor::White,
                move_list: Vec::new(),
                pieces: Board::generate_default_game_pieces(),
                graveyard: Vec::new(),
            }
        }

        fn update(self, pieces: Vec<Piece>, move_list: Vec<Move>) -> Board {
            Board {
                pieces,
                move_list,
                ..self
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

            for file in 0..8 {
                pieces.push(Piece::new(6, file, PieceType::Pawn, PieceColor::Black));
            }
            pieces.push(Piece::new(7, 0, PieceType::Rook, PieceColor::Black));
            pieces.push(Piece::new(7, 1, PieceType::Knight, PieceColor::Black));
            pieces.push(Piece::new(7, 2, PieceType::Bishop, PieceColor::Black));
            pieces.push(Piece::new(7, 3, PieceType::Queen, PieceColor::Black));
            pieces.push(Piece::new(7, 4, PieceType::King, PieceColor::Black));
            pieces.push(Piece::new(7, 5, PieceType::Bishop, PieceColor::Black));
            pieces.push(Piece::new(7, 6, PieceType::Knight, PieceColor::Black));
            pieces.push(Piece::new(7, 7, PieceType::Rook, PieceColor::Black));

            pieces
        }

        fn record_move(&mut self, new_move: &Move) -> Vec<Move> {
            let mut new_move_list = self.move_list.clone();
            new_move_list.push(Move {
                piece_idx: new_move.piece_idx.clone(),
                start_pos: new_move.start_pos.clone(),
                end_pos: new_move.end_pos.clone(),
            });
            new_move_list
        }

        fn is_valid_move(&self, piece: &Piece, dest: &PieceLoc) -> bool {
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

        pub fn move_piece(mut self, new_move: Move) -> Board {
            let selected_piece = self.pieces[new_move.piece_idx];
            println!("Attempting to move piece {selected_piece:?}");
            if self.is_valid_move(&selected_piece, &new_move.end_pos) {
                let new_move_list = self.record_move(&new_move);
                let mut new_pieces = self.pieces.clone();
                new_pieces[new_move.piece_idx].pos = new_move.end_pos;
                return self.update(new_pieces, new_move_list);
            }
            self
        }

        // Takes in a piece location and optionally returns the piece's index if exists
        pub fn piece_exists_at_location(&self, loc: PieceLoc) -> Option<PieceResult> {
            if let Some(piece_idx) = self.get_piece_index_by_loc(loc) {
                Some(PieceResult {
                    piece: self.pieces[piece_idx],
                    index: piece_idx,
                })
            } else {
                None
            }
        }

        fn get_piece_index_by_loc(&self, loc: PieceLoc) -> Option<usize> {
            self.pieces.iter().position(|piece| piece.pos == loc)
        }
    }

    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut chessboard = vec!['.'; (self.ranks * self.files) as usize];
            for piece in &self.pieces {
                let (x, y) = (piece.pos.rank as usize, piece.pos.file as usize);

                let display_char;
                match piece.piece_type {
                    PieceType::Pawn => display_char = 'P',
                    PieceType::Knight => display_char = 'N',
                    PieceType::Bishop => display_char = 'B',
                    PieceType::Rook => display_char = 'R',
                    PieceType::Queen => display_char = 'Q',
                    PieceType::King => display_char = 'K',
                }

                chessboard[(x * (self.ranks as usize)) + y] = display_char;
            }

            let mut output: String = "".to_string();
            for rank in chessboard.chunks(self.ranks.into()).rev() {
                for square in 0..self.files as usize {
                    output.push(rank[square]);
                    output.push(' ');
                }
                output.push('\n');
            }

            write!(f, "{}", output)
        }
    }
}
