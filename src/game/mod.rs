mod renderer;
mod frame;
pub mod controls;

use crate::snake::player::Player;
use self::{renderer::Renderer, controls::get_input};

pub struct Game<'a> {
  player: Player<'a>,
  renderer: Renderer<'a>,
  size: (u16, u16)
}

impl<'a> Game<'a> {
  pub fn new(size: (u16, u16)) -> Game<'a> {
    let player = Player::new();
    let renderer = Renderer::new(size);
    Game { player, renderer, size }
  }

  pub fn start(&self) {
      self.next();
  }

  fn next(&self) {
    // match get_input() {
    //   Some(x) => self.player.handle_move(x),
    //   None => self.player.handle_idle()
    // }

    // self.renderer.render();
  }
}

