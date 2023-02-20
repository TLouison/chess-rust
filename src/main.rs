mod cli;
mod game;

fn main() {
    let mut game = game::board::Board::new();

    loop {
        if let Some(new_move) = cli::prompt_make_move(&game) {
            game = game.move_piece(new_move);
        }

        println!("{:?}", game.move_list);
    }
}
