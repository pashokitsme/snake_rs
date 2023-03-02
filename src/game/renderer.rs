use crate::utils::console::{clear, cursor};
use crossterm::execute;
use std::io::{stdout, Write};

use super::{bonuses::BonusProvider, player::Player};

pub struct Renderer {
  pub size: (i16, i16),
  pub default_buf: Vec<Vec<u8>>,
  pub buf: Vec<Vec<u8>>,
}

impl Renderer {
  pub fn new(size: (i16, i16)) -> Renderer {
    clear::all();
    cursor::to(0, 0);
    let buf = Renderer::get_default_buf(size);
    execute!(stdout(), crossterm::cursor::Hide).unwrap();
    let r = Renderer { size, default_buf: buf.clone(), buf };
    r.display_buf();
    r
  }

  pub fn render(&mut self, player: &Player, bonuses: &BonusProvider) {
    self.prepare();
    bonuses.render(&mut self.buf);
    player.render(&mut self.buf);
    self.display_buf();
    self.display_hud(player);

    stdout().flush().unwrap();
  }

  fn display_buf(&self) {
    let mut d_buf: Vec<u8> = vec![];
    for line in &self.buf {
      d_buf.extend(line);
      d_buf.push(b'\n')
    }

    stdout().write_all(&d_buf).unwrap();
  }

  fn display_hud(&self, player: &Player) {
    println!("Очки: {}. Длина ужика: {}", player.score, player.parts.len());
  }

  fn prepare(&mut self) {
    cursor::to(0, 0);
    clear::under_cursor();
    self.buf = self.default_buf.clone();
  }

  #[allow(clippy::needless_range_loop)]
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
