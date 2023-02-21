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

    #[derive(Copy, Debug, Clone, PartialEq)]
    pub enum PieceColor {
        Black,
        White,
    }

    impl PieceColor {
        pub fn flip(self) -> PieceColor {
            match self {
                Self::Black => Self::White,
                Self::White => Self::Black,
            }
        }
    }

    impl fmt::Display for PieceColor {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
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

    #[derive(Copy, Clone)]
    pub struct Piece {
        pub piece_type: PieceType,
        pub color: PieceColor,
    }

    impl Piece {
        pub fn new(p_type: PieceType, color: PieceColor) -> Piece {
            Piece {
                piece_type: p_type,
                color,
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
                .field("Type", &self.piece_type)
                .field("Color", &self.color)
                .finish()
        }
    }
}

pub mod board {
    use core::fmt;

    use super::piece::{Piece, PieceColor, PieceLoc, PieceType};

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
    }

    #[derive(Clone, Debug)]
    pub struct Board {
        ranks: u8,
        files: u8,
        pub current_turn: PieceColor,
        pub move_list: Vec<Move>,
        pub board: Vec<Option<Piece>>,
        pub graveyard: Vec<Piece>,
    }

    impl Board {
        pub fn new() -> Board {
            Board {
                ranks: 8,
                files: 8,
                current_turn: PieceColor::White,
                move_list: Vec::new(),
                board: Board::generate_default_board(8, 8),
                graveyard: Vec::new(),
            }
        }

        fn update(self, board: Vec<Option<Piece>>, move_list: Vec<Move>) -> Board {
            Board {
                board,
                move_list,
                current_turn: self.current_turn.flip(),
                ..self
            }
        }

        fn generate_default_board(ranks: usize, files: usize) -> Vec<Option<Piece>> {
            let mut board: Vec<Option<Piece>> = vec![None; ranks * files];

            // Generate White Pieces
            board[0] = Some(Piece::new(PieceType::Rook, PieceColor::White));
            board[1] = Some(Piece::new(PieceType::Knight, PieceColor::White));
            board[2] = Some(Piece::new(PieceType::Bishop, PieceColor::White));
            board[3] = Some(Piece::new(PieceType::Queen, PieceColor::White));
            board[4] = Some(Piece::new(PieceType::King, PieceColor::White));
            board[5] = Some(Piece::new(PieceType::Bishop, PieceColor::White));
            board[6] = Some(Piece::new(PieceType::Knight, PieceColor::White));
            board[7] = Some(Piece::new(PieceType::Rook, PieceColor::White));

            // Generate White Pawns
            for file in 0..8 {
                board[8 + file] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
            }

            // Generate Black Pawns
            for file in 0..8 {
                board[48 + file] = Some(Piece::new(PieceType::Pawn, PieceColor::Black));
            }

            // Generate Black Piecces
            board[56] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
            board[57] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
            board[58] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
            board[59] = Some(Piece::new(PieceType::Queen, PieceColor::Black));
            board[60] = Some(Piece::new(PieceType::King, PieceColor::Black));
            board[61] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
            board[62] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
            board[63] = Some(Piece::new(PieceType::Rook, PieceColor::Black));

            board
        }

        fn record_move(&mut self, new_move: &Move) -> Vec<Move> {
            let mut new_move_list = self.move_list.clone();
            new_move_list.push(Move {
                piece: new_move.piece.clone(),
                start_pos: new_move.start_pos.clone(),
                end_pos: new_move.end_pos.clone(),
            });
            new_move_list
        }

        fn is_valid_move(&self, piece: &Piece, start: &PieceLoc, dest: &PieceLoc) -> bool {
            if dest.file >= self.files
                || dest.rank >= self.ranks
                || (dest.rank == start.rank && dest.file == start.file)
            {
                return false;
            }

            if let Some(existing_piece) = self.get_piece_at_location(*dest) {
                if existing_piece.color == piece.color {
                    println!(
                        "Cannot move piece to location occupied by another one of your pieces."
                    );
                    return false;
                }
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
            let selected_piece = new_move.piece;
            if self.current_turn != selected_piece.color {
                println!(
                    "You cannot move that piece, it is currently {}'s turn.",
                    self.current_turn
                );
                return self;
            }
            println!("Attempting to move piece {selected_piece:?}");
            if self.is_valid_move(&selected_piece, &new_move.start_pos, &new_move.end_pos) {
                let new_move_list = self.record_move(&new_move);
                let mut new_board = self.board.clone();

                let start_board_idx = self.get_board_index_from_loc(new_move.start_pos);
                let end_board_idx = self.get_board_index_from_loc(new_move.end_pos);
                new_board[end_board_idx] = new_board[start_board_idx];
                new_board[start_board_idx] = None;

                return self.update(new_board, new_move_list);
            }
            self
        }

        pub fn get_piece_at_location(&self, loc: PieceLoc) -> Option<Piece> {
            let board_index = self.get_board_index_from_loc(loc);
            self.board[board_index]
        }

        fn get_board_index_from_loc(&self, loc: PieceLoc) -> usize {
            ((loc.rank * self.ranks) + loc.file).into()
        }
    }

    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut output: String = "".to_string();
            for rank in self.board.chunks(self.ranks.into()).rev() {
                for square in rank {
                    let display_char;
                    match square {
                        Some(piece) => match piece.piece_type {
                            PieceType::Pawn => display_char = 'P',
                            PieceType::Knight => display_char = 'N',
                            PieceType::Bishop => display_char = 'B',
                            PieceType::Rook => display_char = 'R',
                            PieceType::Queen => display_char = 'Q',
                            PieceType::King => display_char = 'K',
                        },
                        None => display_char = '.',
                    }

                    output.push(display_char);
                    output.push(' ');
                }
                output.push('\n')
            }

            write!(f, "{}", output)
        }
    }
}
