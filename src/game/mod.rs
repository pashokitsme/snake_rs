mod renderer;
pub mod controls;

use crate::snake::player::Player;
use self::{renderer::Renderer, controls::get_input};

pub struct Game<'a> {
  player: Player<'a>,
  renderer: Renderer,
}

impl<'a> Game<'a> {
  pub fn new(size: (u16, u16)) -> Game<'a> {
    let player = Player::new(size);
    let renderer = Renderer::new(size);
    Game { player, renderer }
  }

  pub fn start(&self) {
      self.next();
      self.renderer.render();
  }

  fn next(&self) {
    match get_input() {
      Some(x) => self.player.tick_move(x),
      None => self.player.tick_idle()
    }
    
    self.renderer.render();
  }
}

