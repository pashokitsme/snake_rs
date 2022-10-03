use game::Game;

mod game;
mod snake;
mod utils;

fn main() {
  let game = Game::new((64, 24));
  game.start();
}
