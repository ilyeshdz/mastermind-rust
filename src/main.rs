mod game;
mod options;
mod ui;

use game::Game;
use options::Options;

fn main() {
    let options = Options::from_args();
    let mut game = Game::new(options);
    game.play();
}
