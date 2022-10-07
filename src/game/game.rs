use crate::snake::player::Player;
use super::renderer::Renderer;

pub struct Game {
  player: Player,
  renderer: Renderer,
  total_ticks: u128
}

impl Game {
  pub fn new(size: (i16, i16)) -> Game {
    let player = Player::new(size);
    let renderer = Renderer::new(size);
    Game { player, renderer, total_ticks: 0 }
  }

  pub fn start(&mut self) {
      self.renderer.render(&self.player);
      loop {
        self.next();
      }
  }

  fn tick(&mut self) {
    self.player.tick(self.total_ticks);
    self.total_ticks += 1;
  }

  fn next(&mut self) {
    self.tick();
    self.renderer.render(&self.player);
  }
}