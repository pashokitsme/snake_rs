use std::io::stdout;

use crossterm::{execute};

use crate::utils::console::{cursor, clear};
use super::frame::Frame;

pub struct Renderer<'a> {
  pub size: (u16, u16),
  default_buf: &'a [u8]
}

impl Renderer<'_> {
  pub fn render(&self, frame: &Frame) {
    self.prepare();
  }
  
  fn prepare(&self) {
    cursor::to(0, 0);
    clear::under_cursor();
  }

  pub fn new(size: (u16, u16)) -> Renderer<'static> {
    todo!();

    clear::all();
    cursor::to(0, 0);
    let default_buf = Renderer::get_frame(size).as_bytes();
    execute!(stdout(), crossterm::cursor::Hide).unwrap();
    Renderer { size, default_buf }
  }

  fn get_frame(size: (u16, u16)) -> String {
    let mut frame = String::new();
    frame.push('┏');
    frame.push_str(&str::repeat("─", (size.0 - 2) as usize));
    frame.push('┓');

    let wall = format!("\n│{}│", str::repeat(" ", (size.0 - 2) as usize)).repeat((size.1 - 2) as usize);
    frame.push_str(&wall);

    frame.push('\n');
    frame.push('┗');
    frame.push_str(&str::repeat("─", (size.0 - 2) as usize));
    frame.push('┛');
    frame
  }
}

