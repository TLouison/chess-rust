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

    impl fmt::Display for PieceType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self)
        }
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

        pub fn is_valid(rank: u8, file: u8) -> bool {
            // If both values are valid u8's and within the board's size, return a valid location
            if rank <= 7 && file <= 7 {
                true
            } else {
                false
            }
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

    impl fmt::Display for Piece {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} {}", self.color, self.piece_type)
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

    use crate::game::moves::{move_checker, Move};
    use crate::game::piece::{Piece, PieceColor, PieceLoc, PieceType};

    // List of all valid alpha representation of ranks
    static ALPHA_RANKS_UPPER: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

    #[derive(Clone, Debug)]
    pub struct Board {
        pub ranks: u8,
        pub files: u8,
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

        fn update(
            self,
            board: Vec<Option<Piece>>,
            move_list: Vec<Move>,
            graveyard: Vec<Piece>,
        ) -> Board {
            Board {
                board,
                move_list,
                graveyard,
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

        pub fn move_piece(mut self, new_move: Move) -> Board {
            let selected_piece = new_move.piece;
            let is_valid_move = move_checker::is_valid_move(
                &self,
                &selected_piece,
                &new_move.start_pos,
                &new_move.end_pos,
            );
            match is_valid_move {
                Ok(capturing_move) => {
                    let new_move_list = self.record_move(&new_move);
                    let mut new_board = self.board.clone();
                    let mut new_graveyard = self.graveyard.clone();

                    let start_board_idx = self.get_board_index_from_loc(new_move.start_pos);
                    let end_board_idx = self.get_board_index_from_loc(new_move.end_pos);

                    if capturing_move {
                        // If it was a capturing move, we already confirmed a piece exists at end_board_idx,
                        // so unwrap should be safe.
                        new_graveyard.push(new_board[end_board_idx].unwrap().clone());
                    }
                    new_board[end_board_idx] = new_board[start_board_idx];
                    new_board[start_board_idx] = None;

                    return self.update(new_board, new_move_list, new_graveyard);
                }
                Err(error) => {
                    println!("{}", error);
                    self
                }
            }
        }

        pub fn get_piece_at_location(&self, loc: PieceLoc) -> Option<Piece> {
            let board_index = self.get_board_index_from_loc(loc);
            self.board[board_index]
        }

        fn get_board_index_from_loc(&self, loc: PieceLoc) -> usize {
            ((loc.rank * self.ranks) + loc.file).into()
        }

        fn get_graveyard_display(&self) -> String {
            let mut output: String = String::from("Graveyard:");
            self.graveyard.iter().for_each(|piece| {
                output.push('\n');
                output.push_str(format!("{}", piece).as_str())
            });
            output
        }

        fn get_movelist_display(&self) -> String {
            let mut output: String = String::from("Moves:");

            let mut turn = 1;
            let mut white_turn: bool = true;
            self.move_list.iter().for_each(|_move| {
                if white_turn {
                    output.push('\n');
                    output.push_str(format!("{}. ", turn).as_str());
                    white_turn = false;
                } else {
                    turn += 1;
                    white_turn = true;
                }
                output.push_str(format!("{} ", Board::get_move_display(&_move)).as_str());
            });
            output
        }

        fn get_move_display(m: &Move) -> String {
            format!(
                "{}{}{}",
                Board::get_piece_display(&m.piece),
                Board::convert_rank_numeric_to_alpha(m.end_pos.rank)
                    .expect("Somehow converted a rank > 7"),
                m.end_pos.file + 1
            )
        }

        fn get_piece_display(piece: &Piece) -> char {
            match piece.piece_type {
                PieceType::Pawn => return ' ',
                PieceType::Knight => return 'N',
                PieceType::Bishop => return 'B',
                PieceType::Rook => return 'R',
                PieceType::Queen => return 'Q',
                PieceType::King => return 'K',
            }
        }

        fn convert_rank_numeric_to_alpha(rank: u8) -> Option<char> {
            if usize::from(rank) < ALPHA_RANKS_UPPER.len() {
                Some(ALPHA_RANKS_UPPER[rank as usize].clone())
            } else {
                None
            }
        }
    }

    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut output: String = "".to_string();
            for rank in self.board.chunks(self.ranks.into()).rev() {
                for square in rank {
                    let display_char;
                    match square {
                        Some(piece) => display_char = Board::get_piece_display(piece),
                        None => display_char = '.',
                    }

                    output.push(display_char);
                    output.push(' ');
                }
                output.push('\n')
            }

            write!(
                f,
                "{}\n\n{}\n\n{}",
                output,
                self.get_movelist_display(),
                self.get_graveyard_display()
            )
        }
    }
}

pub mod moves {
    use crate::game::piece::{Piece, PieceLoc};

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

    pub mod move_checker {
        use crate::game::board::Board;
        use crate::game::piece::{Piece, PieceColor, PieceLoc, PieceType};
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
                        if dest.file.abs_diff(start.file) != 1
                            || dest.rank.abs_diff(start.rank) != 1
                        {
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
    }
}
