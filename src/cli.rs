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
    if let Ok(position) = get_input("Enter piece position (i.e. A1, E5): ") {
        let position = position.trim();
        let position = PieceLoc::from_notation(position);
        return position;
    }
    println!("Please enter a valid rank and file, from A-H, 1-8.");
    None
}

pub fn prompt_make_move(game: &Board) -> Option<Move> {
    if let Some(location) = prompt_location() {
        if let Some(piece) = game.get_piece_at_location(location) {
            println!("Piece found: {:?}", piece);

            if let Some(target_location) = prompt_location() {
                println!("Target chosen.");
                if let Ok(new_move) = Move::new(game, &piece, &location, &target_location) {
                    return Some(new_move);
                }
            }
        } else {
            println!("No piece found at ({location:?})");
        }
    }
    None
}
