use core::fmt;
use std::collections::HashMap;

use crate::game::moves::{move_checker, Move};
use crate::game::piece::{
    piece_info::{PieceColor, PieceLoc, PieceType},
    Piece,
};

use super::moves::CaptureResult;

#[derive(Clone, Debug)]
pub struct Board {
    pub ranks: u8,
    pub files: u8,
    pub current_turn: PieceColor,
    pub move_list: Vec<Move>,
    pub board: Vec<Option<Piece>>,
    pub graveyard: HashMap<PieceColor, HashMap<PieceType, u8>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            ranks: 8,
            files: 8,
            current_turn: PieceColor::White,
            move_list: Vec::new(),
            board: Board::generate_default_board(8, 8),
            graveyard: Board::generate_empty_graveyard(),
        }
    }

    fn update(
        self,
        board: Vec<Option<Piece>>,
        move_list: Vec<Move>,
        graveyard: HashMap<PieceColor, HashMap<PieceType, u8>>,
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

    fn generate_empty_graveyard() -> HashMap<PieceColor, HashMap<PieceType, u8>> {
        HashMap::from([
            (
                PieceColor::White,
                HashMap::from([
                    (PieceType::Pawn, 0),
                    (PieceType::Knight, 0),
                    (PieceType::Bishop, 0),
                    (PieceType::Rook, 0),
                    (PieceType::Queen, 0),
                ]),
            ),
            (
                PieceColor::Black,
                HashMap::from([
                    (PieceType::Pawn, 0),
                    (PieceType::Knight, 0),
                    (PieceType::Bishop, 0),
                    (PieceType::Rook, 0),
                    (PieceType::Queen, 0),
                ]),
            ),
        ])
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

    fn handle_move_piece_to_graveyard(
        &mut self,
        capture_type: &CaptureResult,
        m: &Move,
    ) -> HashMap<PieceColor, HashMap<PieceType, u8>> {
        match capture_type {
            CaptureResult::Normal | CaptureResult::EnPassant => {
                if let Some(captured_piece_idx) = self.get_captured_piece_idx(capture_type, m) {
                    if let Some(captured_piece) = self.board[captured_piece_idx] {
                        let mut new_graveyard = self.graveyard.clone();
                        let color_grave = new_graveyard
                            .get_mut(&captured_piece.color)
                            .expect("Didn't find color in graveyard");
                        let piece_grave = color_grave.entry(captured_piece.piece_type).or_insert(1);
                        *piece_grave += 1;

                        self.board[captured_piece_idx] = None;
                        return new_graveyard;
                    } else {
                        panic!("Somehow captured piece that didn't exist.");
                    }
                }
            }
            _ => (),
        }
        // Fallback to returning the old graveyard if no capture happened
        self.graveyard.clone()
    }

    fn get_captured_piece_idx(&self, result: &CaptureResult, m: &Move) -> Option<usize> {
        match result {
            CaptureResult::Normal => Some(self.get_board_index_from_loc(m.end_pos)),
            CaptureResult::EnPassant => match m.piece.color {
                PieceColor::White => Some(self.get_board_index_from_loc(PieceLoc {
                    rank: m.end_pos.rank - 1,
                    file: m.end_pos.file,
                })),
                PieceColor::Black => Some(self.get_board_index_from_loc(PieceLoc {
                    rank: m.end_pos.rank + 1,
                    file: m.end_pos.file,
                })),
            },
            CaptureResult::None => None,
        }
    }

    fn handle_moving_piece(&self, m: &Move) -> Vec<Option<Piece>> {
        let mut new_board = self.board.clone();
        let mut selected_piece = m.piece;

        let start_board_idx = self.get_board_index_from_loc(m.start_pos);
        let end_board_idx = self.get_board_index_from_loc(m.end_pos);

        selected_piece.has_moved = true;
        new_board[end_board_idx] = Some(selected_piece);
        new_board[start_board_idx] = None;

        // Update the moved piece's has_moved flag
        if let Some(_piece) = new_board[end_board_idx] {
            new_board[end_board_idx].unwrap().has_moved = true;
        }
        new_board
    }

    pub fn move_piece(mut self, new_move: Move) -> Board {
        let is_valid_move = move_checker::is_valid_move(
            &self,
            &new_move.piece,
            &new_move.start_pos,
            &new_move.end_pos,
        );
        match is_valid_move {
            Ok(capturing_move) => {
                let new_move_list = self.record_move(&new_move);
                let new_board = self.handle_moving_piece(&new_move);
                let new_graveyard = self.handle_move_piece_to_graveyard(&capturing_move, &new_move);

                return self.update(new_board, new_move_list, new_graveyard);
            }
            Err(error) => {
                println!("{}", error);
                self
            }
        }
    }

    pub fn get_previous_move(&self) -> Option<Move> {
        self.move_list.last().cloned()
    }

    pub fn get_piece_at_location(&self, loc: PieceLoc) -> Option<Piece> {
        let board_index = self.get_board_index_from_loc(loc);
        self.board.get(board_index).copied().unwrap_or_else(|| None)
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
                    Some(piece) => display_char = board_display::get_piece_display(piece, false),
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
            board_display::get_movelist_display(self),
            board_display::get_graveyard_display(self),
        )
    }
}

pub mod board_display {
    // List of all valid alpha representation of ranks
    static ALPHA_RANKS_UPPER: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

    use super::Board;
    use crate::game::piece::{
        piece_info::{PieceColor, PieceType},
        Piece,
    };

    pub fn get_graveyard_display(board: &Board) -> String {
        let mut output: String = String::from("Graveyard:");
        let mut piece_display: [String; 2] = [
            String::from("\n\tWhite pieces:"),
            String::from("\n\tBlack pieces:"),
        ];
        for (color, piece_type) in &board.graveyard {
            let display_index;
            match color {
                PieceColor::White => display_index = 0,
                PieceColor::Black => display_index = 1,
            };
            let mut found_captured_of_color = false;
            for (p_type, &captured) in piece_type {
                if captured > 0 {
                    piece_display[display_index]
                        .push_str(format!("\n\t\t{}x {}", captured, p_type).as_str());
                    found_captured_of_color = true;
                }
            }
            if !found_captured_of_color {
                piece_display[display_index].push_str("\n\t\tNo pieces have been captured yet.");
            }
        }
        piece_display.iter().for_each(|color| {
            output.push_str(color.as_str());
        });
        output
    }

    pub fn get_movelist_display(board: &Board) -> String {
        let mut output: String = String::from("Moves:");

        let mut turn = 1;
        let mut white_turn: bool = true;
        board.move_list.iter().for_each(|_move| {
            if white_turn {
                output.push('\n');
                output.push_str(format!("{}. ", turn).as_str());
                white_turn = false;
            } else {
                turn += 1;
                white_turn = true;
            }
            output.push_str(format!("{} ", &_move.get_move_display()).as_str());
        });
        output
    }

    pub fn get_piece_display(piece: &Piece, pawn_blank: bool) -> char {
        match piece.piece_type {
            PieceType::Pawn => {
                if pawn_blank {
                    return ' ';
                } else {
                    return 'P';
                }
            }
            PieceType::Knight => return 'N',
            PieceType::Bishop => return 'B',
            PieceType::Rook => return 'R',
            PieceType::Queen => return 'Q',
            PieceType::King => return 'K',
        }
    }

    pub fn convert_rank_alpha_to_numeric(rank: char) -> Option<u8> {
        let rank = rank.clone().to_ascii_uppercase();

        if ALPHA_RANKS_UPPER.contains(&rank) {
            match ALPHA_RANKS_UPPER.iter().position(|c| c == &rank) {
                Some(i) => return Some(i as u8),
                None => return None,
            };
        }
        None
    }

    pub fn convert_rank_numeric_to_alpha(rank: u8) -> Option<char> {
        let rank = usize::from(rank);
        if rank < ALPHA_RANKS_UPPER.len() {
            ALPHA_RANKS_UPPER.get(rank).copied()
        } else {
            None
        }
    }
}
