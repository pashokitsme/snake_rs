use super::{renderer::Renderer, player::Player};

pub struct Game {
  player: Player,
  renderer: Renderer
}

impl Game {
  pub fn new(size: (i16, i16), init_parts: usize) -> Game {
    let player = Player::new(size, init_parts);
    let renderer = Renderer::new(size);
    Game { player, renderer }
  }

  pub fn start(&mut self) {
      self.renderer.render(&self.player);
      loop {
        self.next();
      }
  }

  fn tick(&mut self) {
    self.player.tick();
  }

  fn next(&mut self) {
    self.tick();
    self.renderer.render(&self.player);
  }
}