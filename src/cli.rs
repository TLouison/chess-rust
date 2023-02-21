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
        get_input("Enter piece rank (1-8): "),
        get_input("Enter piece file (1-8): "),
    ) {
        let rank = (rank.trim().parse::<u8>().unwrap()) - 1;
        let file = (file.trim().parse::<u8>().unwrap()) - 1;
        return Some(PieceLoc { rank, file });
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
