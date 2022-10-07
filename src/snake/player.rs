use crate::utils;
use crate::game::controls;

pub struct Part {
  pub pos: (i16, i16),
  pub symbol: u8
}

pub struct Player {
  pub direction: (i16, i16),
  field_size: (i16, i16),
  parts: Vec<Part>
}

impl Player {
  pub fn new(size: (i16, i16)) -> Player {
    let mut pl = Player { direction: (0, 0), field_size: size, parts: Vec::new() };
    pl.parts.push(Part { pos: (size.0 / 2, size.1 / 2), symbol: b'O' });
    pl
  }

  // pub fn add_part(&mut self) {
  //   let last = self.parts.last().unwrap();
  //   let part = Part { pos: last.pos, symbol: b'o' };
  //   self.parts.push(part);
  // }

  pub fn render(&self, buf: &mut Vec<Vec<u8>>) {
    for part in &self.parts {
      let pos = part.pos;
      buf[pos.1 as usize - 1][pos.0 as usize - 1] = part.symbol;
    }
  }

  pub fn tick(&mut self) {
    self.tick_move(controls::get_input());
    self.direction = (0, 0);
  }

  fn tick_move(&mut self, new_direction: Option<(i16, i16)>) {
    match new_direction {
      Some(x) => self.direction = x,
      None => {},
    }

    for part in &mut self.parts {
      part.pos = utils::wrapped_pos(self.field_size , (self.direction.0 + part.pos.0, self.direction.1 + part.pos.1));
    }
  }
}