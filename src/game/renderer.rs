use std::io::{stdout, Write};

use crossterm::execute;
use crate::utils::console::{cursor, clear};

pub struct Renderer {
  pub size: (u16, u16),
  buf: Vec<u8>,
}

impl Renderer {
  pub fn render(&self) {
    self.prepare();
    self.display_buf();
  }
  
  fn display_buf(&self) {
    self.prepare();
    stdout().write_all(&self.buf)
        .expect("Error while displaying buf");
    stdout().flush().unwrap();
  }
  
  fn prepare(&self) {
    cursor::to(0, 0);
    clear::under_cursor();
  }

  pub fn new(size: (u16, u16)) -> Renderer {
    clear::all();
    cursor::to(0, 0);
    let buf = Renderer::get_frame(size).as_bytes().to_owned();
    execute!(stdout(), crossterm::cursor::Hide).unwrap();
    let r = Renderer { size, buf };
    r.display_buf();
    r
  }

  fn get_frame(size: (u16, u16)) -> String {
    let mut frame = String::new();
    frame.push('┏');
    frame.push_str(&str::repeat("─", (size.0 - 2) as usize));
    frame.push('┓');
    frame.push_str(&format!("\n│{}│", str::repeat(" ", (size.0 - 2) as usize)).repeat((size.1 - 2) as usize));
    frame.push('\n');
    frame.push('┗');
    frame.push_str(&str::repeat("─", (size.0 - 2) as usize));
    frame.push('┛');
    frame
  }
}

