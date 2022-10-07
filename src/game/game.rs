use crate::snake::player::Player;
use super::renderer::Renderer;

pub struct Game {
  player: Player,
  renderer: Renderer,
}

impl Game {
  pub fn new(size: (i16, i16)) -> Game {
    let player = Player::new(size);
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