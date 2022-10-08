use std::io::{stdout, Write};
use crossterm::execute;
use crate::utils::console::{cursor, clear};

use super::player::Player;

pub struct Renderer {
  pub size: (i16, i16),
  pub default_buf: Vec<Vec<u8>>,
  pub buf: Vec<Vec<u8>>,
}

impl Renderer {
  pub fn render(&mut self, player: &Player) {
    self.prepare();
    player.render(&mut self.buf);
    self.display_buf();
  }
  
  fn display_buf(&self) {
    let mut d_buf: Vec<u8> = vec![];
    for line in &self.buf {
      d_buf.extend(line);
      d_buf.push(b'\n')
    }

    stdout().write_all(&d_buf)
            .expect("Error while displaying buf");
    stdout().flush()
        .expect("Error while flushing display");
  }
  
  fn prepare(&mut self) {
    cursor::to(0, 0);
    clear::under_cursor();
    self.buf = self.default_buf.clone();
  }

  pub fn new(size: (i16, i16)) -> Renderer {
    clear::all();
    cursor::to(0, 0);
    let buf = Renderer::get_default_buf(size);
    execute!(stdout(), crossterm::cursor::Hide).unwrap();
    let r = Renderer { size, default_buf: buf.clone(), buf };
    r.display_buf();
    r
  }

  fn get_default_buf(size: (i16, i16)) -> Vec<Vec<u8>> {
    let size = (size.0 as usize, size.1 as usize);
    let mut buf = vec![vec![b' '; size.0]; size.1];

    buf[0][0] = b'+';
    buf[0][size.0 - 1] = b'+';
    for i in 1..size.0 - 1 {
      buf[0][i] = b'-';
    }

    for i in 1..size.1 {
      buf[i][0] = b'|';
      buf[i][size.0 - 1] = b'|';
    }

    buf[size.1 - 1][0] = b'+';
    buf[size.1 - 1][size.0 - 1] = b'+';
    for i in 1..size.0 - 1 {
      buf[size.1 - 1][i] = b'-';
    }

    buf
  }
}

