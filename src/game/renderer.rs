use crate::utils::console::{clear, cursor};
use crossterm::execute;
use std::io::{stdout, Write};

use super::{bonuses::EffectSpawner, player::Player};

pub trait Render {
  fn render(&self, buffer: &mut [Vec<u8>]);
}

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
    r.print_buffer();
    r
  }

  pub fn render(&mut self, player: &Player, bonuses: &EffectSpawner) {
    self.prepare();
    bonuses.render(&mut self.buf);
    player.render(&mut self.buf);
    self.print_buffer();
    self.print_hud(player);

    stdout().flush().unwrap();
  }

  fn print_buffer(&self) {
    let mut d_buf: Vec<u8> = vec![];
    for line in &self.buf {
      d_buf.extend(line);
      d_buf.push(b'\n')
    }

    stdout().flush().expect("Couldn't flush");
    stdout().lock().write_all(&d_buf).expect("Couldn't write to stdout");
    stdout().flush().expect("Couldn't flush");
  }

  fn print_hud(&self, player: &Player) {
    println!("Очки: {}. Длина: {}", player.score, player.parts.len());
  }

  fn prepare(&mut self) {
    clear::upper_cursor();
    cursor::to(0, 0);
    self.buf = self.default_buf.clone();
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
