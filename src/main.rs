use game::piece::PieceLoc;

mod game;

fn main() {
    println!("Hello, world!");

    let current_game = game::game::Game::new();
    let current_pieces = &current_game.board.pieces;
    for (i, piece) in current_pieces.iter().enumerate() {
        println!("{i} {piece:?}");
    }

    let kings_pawn = current_pieces[12];
    current_game
        .board
        .move_piece(&kings_pawn, PieceLoc::new(4, 4));
}
