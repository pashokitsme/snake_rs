mod game;
mod utils;

use game::{Game, InitSettings};

fn main() {
  let init = InitSettings {
    field_size: (64, 24),
    input_timeout: (50, 110),
    parts_count: 4
  };

  let mut game = Game::new(init);
  game.start();
}