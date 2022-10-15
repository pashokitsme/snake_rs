use crate::utils;
use crate::game::controls;

#[derive(Clone, Copy)]
pub struct Part {
  pub pos: (i16, i16),
  pub dir: (i16, i16),
  pub prev_dir: (i16, i16),
  pub symbol: u8
}

impl Part {
    pub fn set_dir(&mut self, new: (i16, i16)) {
      self.prev_dir = self.dir;
      self.dir = new;
    }
}

pub struct Player {
  field_size: (i16, i16),
  parts: Vec<Part>
}

impl Player {
  pub fn new(size: (i16, i16), parts: usize) -> Player {
    let head = Part { pos: (size.0 / 2, size.1 / 2), symbol: b'O', dir: (1, 0), prev_dir: (1, 0) };
    let mut pl = Player { field_size: size, parts: vec![head] };
    for _ in 0..parts {
      pl.add_part()
    }
    pl
  }

  pub fn add_part(&mut self) {
    let last = self.tail();
    let pos = (last.pos.0 + (self.head().dir.0 * -1), last.pos.1 + (self.head().dir.1 * -1));
    for part in &self.parts {
      if part.pos == pos {
        return;
      }
    }
    self.parts.push(Part { pos, symbol: b'o', dir: last.dir, prev_dir: last.prev_dir });
  }

  pub fn render(&self, buf: &mut Vec<Vec<u8>>) {
    for part in &self.parts {
      let pos = part.pos;
      buf[pos.1 as usize - 1][pos.0 as usize - 1] = part.symbol;
    }
  }

  pub fn tick(&mut self) {
    let timeout = match self.head().dir.1 {
      0 => 50,
      _ => 110
    };
    let input = controls::get_input(timeout);
    if self.tick_head(input) {
      self.tick_move();
    }
  }

  fn tick_move(&mut self) {
    for i in 1..self.parts.len() {
      let slice = self.parts.split_at_mut(i);
      let mut this = slice.1.first_mut().unwrap();
      let prev = slice.0.last().unwrap();
      this.set_dir(prev.prev_dir);

      this.pos = utils::wrapped_pos(self.field_size, (this.dir.0 + this.pos.0, this.dir.1 + this.pos.1));
    }
  }

  fn tick_head(&mut self, new_dir: Option<(i16, i16)>) -> bool {
    match new_dir {
      Some(x) => {
        self.head_mut().set_dir(x);
      },
      None => return false,
    }
    let head = self.head().to_owned();
    let desired_pos = utils::wrapped_pos(self.field_size, (head.dir.0 + head.pos.0, head.dir.1 + head.pos.1));

    if self.is_occupied(desired_pos) {
      panic!("Loss")
    }

    self.head_mut().pos = desired_pos;
    true
  }

  pub fn is_occupied(&self, desired_pos: (i16, i16)) -> bool {
    for part in self.parts.iter() {
      if part.pos == desired_pos {
        return true
      }
    }

    false
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