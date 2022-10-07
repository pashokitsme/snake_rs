use crate::snake::player::Player;
use super::{renderer::Renderer};

pub struct Game<'a> {
  player: Player<'a>,
  renderer: Renderer,
}

impl<'a> Game<'a> {
  pub fn new(size: (i16, i16)) -> Game<'a> {
    let player = Player::new(size);
    let renderer = Renderer::new(size);
    Game { player, renderer }
  }

  pub fn start(&mut self) {
      self.renderer.render(&self.player);
      self.next();
  }

  fn tick(&mut self) {
    self.player.tick();
  }

  fn next(&mut self) {
    self.tick();
    self.renderer.render(&self.player);
  }
}