pub struct Part<'a> {
  pub pos: (u16, u16),
  pub symbol: &'a char
}

pub struct Player<'a> {
  pub direction: (u16, u16),
  parts: Vec<Part<'a>>
}

impl Player<'_> {
  pub fn add_part(&mut self) {
    let last = self.parts.last().unwrap();
    let part = Part { pos: last.pos, symbol: &'o' };
    self.parts.push(part);
  }

  pub fn new() -> Player<'static> {
    let mut pl = Player { direction: (0, 0), parts: Vec::new() };
    pl.parts.push(Part { pos: (0, 0), symbol: &'O' });
    pl
  }

  pub fn handle_move(&self, direction: (i16, i16)) {
    
  }

  pub fn handle_idle(&self) {

  }
}