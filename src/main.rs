use game::Game;

mod board;
mod player;
mod game;
mod utils {
    pub mod printer;
}

/// Game Entrypoint
fn main() {
    let game: Game = Game::new();
    game.start();
}
