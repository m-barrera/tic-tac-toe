use game::Game;

mod board;
mod player;
mod game;
mod utils {
    pub mod printer;
}

fn main() {
    let game: Game = Game::new();
    game.start();
}
