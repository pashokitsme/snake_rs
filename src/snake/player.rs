pub struct Part<'a> {
  pub pos: (u16, u16),
  pub symbol: &'a char
}

pub struct Player<'a> {
  pub direction: (u16, u16),
  field_size: (u16, u16),
  parts: Vec<Part<'a>>
}

impl Player<'_> {
  pub fn add_part(&mut self) {
    let last = self.parts.last().unwrap();
    let part = Part { pos: last.pos, symbol: &'o' };
    self.parts.push(part);
  }

  

  pub fn tick_move(&self, direction: (i16, i16)) {
    
  }

  pub fn tick_idle(&self) {

  }
}