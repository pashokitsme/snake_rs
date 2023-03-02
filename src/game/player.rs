use crate::utils::{self, controls::get_input};

use super::{renderer::Render, InitSettings};

#[derive(Clone, Copy)]
pub struct Part {
  pub pos: (i16, i16),
  pub dir: (i16, i16),
  pub prev_dir: (i16, i16),
  symbol: u8,
}

impl Part {
  pub fn set_dir(&mut self, new: (i16, i16)) {
    self.prev_dir = self.dir;
    self.dir = new;
  }
}

impl Render for Part {
  fn render(&self, buffer: &mut [Vec<u8>]) {
    buffer[(self.pos.1 - 1) as usize][(self.pos.0 - 1) as usize] = self.symbol
  }
}

pub struct Player {
  pub parts: Vec<Part>,
  pub score: i32,
  pub is_lost: bool,
  field_size: (i16, i16),
  input_timeout: (u64, u64),
}

impl Render for Player {
  fn render(&self, buf: &mut [Vec<u8>]) {
    self.parts.iter().for_each(|p| p.render(buf))
  }
}

impl Player {
  pub fn new(init: &InitSettings) -> Player {
    let size = init.field_size;
    let head = Part { pos: (size.0 / 2, size.1 / 2), symbol: b'O', dir: (1, 0), prev_dir: (1, 0) };
    let mut pl = Player { field_size: size, parts: vec![head], is_lost: false, score: 0, input_timeout: init.input_timeout };
    for _ in 0..init.parts_count {
      pl.add_part()
    }
    pl
  }

  pub fn add_part(&mut self) {
    let last = self.tail();
    let pos = (last.pos.0 + (-last.dir.0), last.pos.1 + (-last.dir.1));
    if self.parts.iter().any(|p| p.pos == pos) {
      return;
    }
    self
      .parts
      .push(Part { pos, symbol: b'o', dir: last.dir, prev_dir: last.prev_dir });
  }

  pub fn remove_part(&mut self) {
    if self.parts.len() > 1 {
      self.parts.pop();
    }
  }

  pub fn tick(&mut self) {
    let timeout = match self.head().dir.1 {
      0 => self.input_timeout.0,
      _ => self.input_timeout.1,
    };
    let input = get_input(timeout);
    if self.tick_head(input) {
      self.tick_move();
    }
  }

  fn tick_move(&mut self) {
    for i in 1..self.parts.len() {
      let prev = self.parts[i - 1].prev_dir;
      self.parts[i].set_dir(prev);
      self.parts[i].pos = utils::wrapped_pos(
        self.field_size,
        (self.parts[i].dir.0 + self.parts[i].pos.0, self.parts[i].dir.1 + self.parts[i].pos.1),
      );
    }
  }

  fn tick_head(&mut self, new_dir: Option<(i16, i16)>) -> bool {
    match new_dir {
      Some(x) => {
        self.head_mut().set_dir(x);
      }
      None => {
        let head = self.head_mut();
        head.set_dir(head.dir)
      }
    }
    let head = self.head();
    let desired_pos = utils::wrapped_pos(self.field_size, (head.dir.0 + head.pos.0, head.dir.1 + head.pos.1));

    if self.is_occupied(desired_pos).is_some() {
      self.is_lost = true;
      return false;
    }

    self.head_mut().pos = desired_pos;
    true
  }

  pub fn is_occupied(&self, pos: (i16, i16)) -> Option<&Part> {
    self.parts.iter().find(|part| part.pos == pos)
  }

  pub fn head(&self) -> &Part {
    self.parts.first().unwrap()
  }

  fn tail(&self) -> &Part {
    self.parts.last().unwrap()
  }

  fn head_mut(&mut self) -> &mut Part {
    self.parts.first_mut().unwrap()
  }
}
