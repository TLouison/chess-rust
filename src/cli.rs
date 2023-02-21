use crate::game::board::{Board, Move};
use crate::game::piece::PieceLoc;
use std::io::{self, Write};

fn get_input(prompt: &str) -> io::Result<String> {
    let mut buffer = String::new();
    print!("{}", prompt);
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading value from stdin.");
    Ok(buffer)
}

fn prompt_location() -> Option<PieceLoc> {
    if let (Ok(rank), Ok(file)) = (
        get_input("Enter piece rank (0-7): "),
        get_input("Enter piece file (0-7): "),
    ) {
        let rank_int: u8 = rank.trim().to_string().parse().unwrap();
        let file_int: u8 = file.trim().to_string().parse().unwrap();
        println!("{rank_int},{file_int}");
        return Some(PieceLoc {
            rank: rank_int,
            file: file_int,
        });
    }
    None
}

pub fn prompt_make_move(game: &Board) -> Option<Move> {
    if let Some(location) = prompt_location() {
        if let Some(piece_result) = game.piece_exists_at_location(location) {
            println!("Piece found: {:?}", piece_result.piece);
            if let Some(target_location) = prompt_location() {
                println!("Target chosen.");
                return Some(Move::new(piece_result.index, location, target_location));
            }
        } else {
            println!("No piece found at ({location:?})");
        }
    }
    None
}
