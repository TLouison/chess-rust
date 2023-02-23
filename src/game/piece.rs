
use std::fmt;

pub mod piece_info {
    use std::fmt;

    use crate::game::board::board_display;

    #[derive(Copy, Debug, Clone, Hash, Eq, PartialEq)]
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
            write!(f, "{:?}", self)
        }
    }

    #[derive(Copy, Debug, Clone, Eq, Hash, PartialEq)]
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

        pub fn from_notation(notation: &str) -> Option<PieceLoc> {
            if notation.len() != 2 {
                println!("Wrong number of chars");
                return None;
            }

            let mut chars = notation.chars();
            if let (Some(file), Some(rank)) = (
                board_display::convert_rank_alpha_to_numeric(chars.next()?),
                chars.next()?.to_digit(10),
            ) {
                let rank = (rank as u8) - 1;
                if PieceLoc::is_valid(rank, file) {
                    return Some(PieceLoc::new(rank, file));
                }
                println!("Got invalid location");
            }
            println!("Couldn't convert input to PieceLoc");
            None
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

    impl fmt::Debug for PieceLoc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Position")
                .field("Rank", &self.rank)
                .field("File", &self.file)
                .finish()
        }
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: piece_info::PieceType,
    pub color: piece_info::PieceColor,
}

impl Piece {
    pub fn new(p_type: piece_info::PieceType, color: piece_info::PieceColor) -> Piece {
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

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Piece")
            .field("Type", &self.piece_type)
            .field("Color", &self.color)
            .finish()
    }
}
