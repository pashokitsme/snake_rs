mod game;
mod snake;
mod utils;

use game::Game;

fn main() {
  let mut game = Game::new((64, 24));
  game.start();
}