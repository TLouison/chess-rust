use game::piece::PieceLoc;
use std::io::{self, Write};

mod game;

fn get_input(prompt: &str) -> io::Result<String> {
    let mut buffer = String::new();
    print!("{}", prompt);
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading value from stdin.");
    Ok(buffer)
}

fn main() {
    let mut game = game::board::Board::new();

    loop {
        if let (Ok(rank), Ok(file)) = (
            get_input("Enter piece rank (0-7): "),
            get_input("Enter piece file (0-7): "),
        ) {
            let rank_int: u8 = rank.trim().to_string().parse().unwrap();
            let file_int: u8 = file.trim().to_string().parse().unwrap();
            println!("{rank_int},{file_int}");
            if let Some(mut piece) = &game.get_piece_by_loc(PieceLoc {
                rank: rank_int,
                file: file_int,
            }) {
                println!("Piece found: {piece:?}");
                // game.move_piece(&mut piece, PieceLoc { rank: 3, file: 4 });
            } else {
                println!("No piece found there.");
            }
        }
    }
}
