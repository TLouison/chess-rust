use game::piece::PieceLoc;

mod game;

fn main() {
    let game = game::board::Board::new();

    if let Some(mut piece) = &game.get_piece_by_loc(PieceLoc { rank: 1, file: 4 }) {
        println!("Piece found: {piece:?}");
        game.move_piece(&mut piece, PieceLoc { rank: 3, file: 4 });
    } else {
        println!("No piece found there.");
    }
}
