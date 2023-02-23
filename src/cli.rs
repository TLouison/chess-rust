use crate::game::board::Board;
use crate::game::moves::Move;
use crate::game::piece::piece_info::PieceLoc;
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
        let rank = rank.trim().parse::<u8>();
        let file = file.trim().parse::<u8>();

        // If both values are valid u8's and within the board's size, return a valid location
        if let (Some(rank), Some(file)) = (rank.ok(), file.ok()) {
            if PieceLoc::is_valid(rank - 1, file - 1) {
                return Some(PieceLoc::new(rank - 1, file - 1));
            }
        }
    }
    println!("Please enter a valid rank and file, from 1-8.");
    None
}

pub fn prompt_make_move(game: &Board) -> Option<Move> {
    if let Some(location) = prompt_location() {
        if let Some(piece) = game.get_piece_at_location(location) {
            println!("Piece found: {:?}", piece);

            if let Some(target_location) = prompt_location() {
                println!("Target chosen.");
                return Some(Move::new(piece, location, target_location));
            }
        } else {
            println!("No piece found at ({location:?})");
        }
    }
    None
}
