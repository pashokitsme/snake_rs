mod game;
mod utils;

use game::Game;

fn main() {
  let mut game = Game::new((64, 24), 0);
  game.start();
}