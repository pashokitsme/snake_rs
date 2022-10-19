use super::player::Player;

pub enum BonusKind {
  AddPart, 
  RemovePart
}

pub struct Bonus {
  pub pos: (i16, i16),
  pub alive_for_ticks: i16,
  pub used: bool,
  pub kind: BonusKind,
  pub symbol: u8
}

impl Bonus {
  pub fn affect(&mut self, player: &mut Player) {
    match self.kind {
      BonusKind::AddPart => {
        player.add_part();
        player.score += 1;
      },
      BonusKind::RemovePart => { 
        player.remove_part();
        player.score += 2;
      }
    }

    self.used = true;
  }
}

pub struct BonusProvider {
  bonuses: Vec<Bonus>
}

impl BonusProvider {
  pub fn new() -> BonusProvider {
    BonusProvider { bonuses: vec![] }
  }

  pub fn render(&self, buf: &mut [Vec<u8>]) {
    self.bonuses.iter().for_each(|b| buf[(b.pos.1 - 1) as usize][(b.pos.0 - 1) as usize] = b.symbol);
  }

  pub fn place_addpart(&mut self, pos: (i16, i16)) {
    let bonus = Bonus { pos, alive_for_ticks: 32000, used: false, kind: BonusKind::AddPart, symbol: b'@' };
    self.bonuses.push(bonus);
  }

  pub fn place_removepart(&mut self, pos: (i16, i16)) {
    let bonus = Bonus { pos, alive_for_ticks: 32000, used: false, kind: BonusKind::RemovePart, symbol: b'%' };
    self.bonuses.push(bonus);
  }

  pub fn tick(&mut self, player: &mut Player) {
    let head_pos = player.head().pos;
    if let Some(bonus) = self.bonuses.iter_mut().find(|b| b.pos == head_pos) {
      bonus.affect(player);
    }
    self.tick_lifetimes();
  }

  pub fn is_occupied(&self, pos: (i16, i16)) -> Option<&Bonus> {
    self.bonuses.iter().find(|b| b.pos == pos)
  }

  pub fn tick_lifetimes(&mut self) {
    self.bonuses.iter_mut().for_each(|b| b.alive_for_ticks -= 1);
    self.bonuses.retain(|b| !b.used && b.alive_for_ticks > 0);
  }

  pub fn current_count(&self) -> usize {
    self.bonuses.len()
  }
}