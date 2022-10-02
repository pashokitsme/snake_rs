use crate::utils::console::{cursor, clear};

use crate::snake::Vec2;

pub struct Renderer {
  pub size: Vec2<u16>
}

impl Renderer {
  pub fn render(&self) {
    self.prepare();
  }
  
  fn prepare(&self) {
    cursor::to(0, self.size.y);
    clear::above_cursor();
    cursor::to(0, 0);
  }
}

pub fn new(size: Vec2::<u16>) -> Renderer {
  clear::all();
  Renderer { size }
}
