use crate::utils;

pub struct Part<'a> {
  pub pos: (i16, i16),
  pub symbol: &'a char
}

pub struct Player<'a> {
  pub direction: (i16, i16),
  field_size: (i16, i16),
  parts: Vec<Part<'a>>
}

impl<'a> Player<'a> {
  pub fn new(size: (i16, i16)) -> Player<'a> {
    let mut pl = Player { direction: (0, 0), field_size: size, parts: Vec::new() };
    pl.parts.push(Part { pos: (size.0 / 2, size.1 / 2), symbol: &'O' });
    pl
  }

  pub fn add_part(&mut self) {
    let last = self.parts.last().unwrap();
    let part = Part { pos: last.pos, symbol: &'o' };
    self.parts.push(part);
  }

  pub fn render(&self, buf: &mut Vec<Vec<u8>>) {
    for part in &self.parts {
      
    }
  }

  pub fn tick(&mut self) {
    self.tick_move(crate::game::controls::get_input());
  }

  fn tick_move(&mut self, new_direction: Option<(i16, i16)>) {
    match new_direction {
      Some(x) => self.direction = x,
      None => {},
    }

    let dest = utils::wrapped_pos(self.field_size, (self.direction.0 + self.field_size.0, self.direction.1 + self.field_size.1));
    for part in &mut self.parts {
      part.pos = dest;
    }
  }
}