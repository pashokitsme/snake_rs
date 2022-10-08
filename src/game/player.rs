use crate::utils;
use crate::game::controls;

pub struct Part {
  pub pos: (i16, i16),
  pub count: i16,
  pub direction: (i16, i16),
  pub symbol: u8
}

pub struct Player {
  field_size: (i16, i16),
  parts: Vec<Part>
}

impl Player {
  pub fn new(size: (i16, i16)) -> Player {
    let head = Part { pos: (size.0 / 2, size.1 / 2), count: 0, symbol: b'O', direction: (1, 0) };
    let mut pl = Player { field_size: size, parts: vec![head] };
    pl.add_part();
    pl.add_part();
    pl.add_part();
    pl.add_part();
    pl
  }

  pub fn add_part(&mut self) {
    let last = self.tail();
    let pos = (last.pos.0 + (self.head().direction.0 * -1), last.pos.1 + (self.head().direction.1 * -1));
    for part in &self.parts {
      if part.pos == pos {
        return;
      }
    }
    self.parts.push(Part { pos, count: last.count + 1, symbol: b'o', direction: self.head().direction });
  }

  pub fn render(&self, buf: &mut Vec<Vec<u8>>) {
    for part in &self.parts {
      let pos = part.pos;
      buf[pos.1 as usize - 1][pos.0 as usize - 1] = part.symbol;
    }
  }

  pub fn tick(&mut self, num: u128) {
    let timeout = match self.head().direction.1 {
      x if x != 0 => 110,
      _ => 50
    };
    self.tick_move(controls::get_input(timeout), num);
  }

  fn tick_move(&mut self, new_direction: Option<(i16, i16)>, num: u128) {
    match new_direction {
      Some(x) => {
        self.head_mut()
          .direction = x
      },
      None => {},
    }

    for i in 0..self.parts.len() {
      if i > 0 && num % (self.parts[i].count as u128 + 1) == 0 {
        self.parts[i].direction = self.parts[i - 1].direction;
      }
      let mut part = &mut self.parts[i];
      part.pos = utils::wrapped_pos(self.field_size, (part.direction.0 + part.pos.0, part.direction.1 + part.pos.1));
    }
  }

  fn head(&self) -> &Part {
    self.parts.first().unwrap()
  }
  
  fn tail(&self) -> &Part {
    self.parts.last().unwrap()
  }

  fn head_mut(&mut self) -> &mut Part {
    self.parts.first_mut().unwrap()
  }
}