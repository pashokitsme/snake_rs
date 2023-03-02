use super::{player::Player, renderer::Render};

pub enum EffectKind {
  Score(i32),
  RemovePart,
}

impl EffectKind {
  pub fn score(&self) -> i32 {
    match self {
      EffectKind::Score(x) => *x,
      EffectKind::RemovePart => 2,
    }
  }
}

pub struct Effect {
  pub pos: (i16, i16),
  pub alive_for_ticks: i16,
  pub used: bool,
  pub kind: EffectKind,
  symbol: u8,
}

impl Effect {
  pub fn affect(&mut self, player: &mut Player) {
    match self.kind {
      EffectKind::Score(_) => player.add_part(),
      EffectKind::RemovePart => player.remove_part(),
    }
    player.score += self.kind.score();
    self.used = true;
  }
}

impl Render for Effect {
  fn render(&self, buffer: &mut [Vec<u8>]) {
    buffer[(self.pos.1 - 1) as usize][(self.pos.0 - 1) as usize] = self.symbol
  }
}

pub struct EffectSpawner {
  bonuses: Vec<Effect>,
}

impl EffectSpawner {
  pub fn new() -> EffectSpawner {
    EffectSpawner { bonuses: vec![] }
  }

  pub fn render(&self, buf: &mut [Vec<u8>]) {
    self.bonuses.iter().for_each(|b| b.render(buf));
  }

  pub fn add(&mut self, pos: (i16, i16), kind: EffectKind) {
    let bonus = Effect { pos, alive_for_ticks: 32000, used: false, kind, symbol: b'@' };
    self.bonuses.push(bonus);
  }

  pub fn remove(&mut self, pos: (i16, i16)) {
    let bonus = Effect { pos, alive_for_ticks: 32000, used: false, kind: EffectKind::RemovePart, symbol: b'%' };
    self.bonuses.push(bonus);
  }

  pub fn tick(&mut self, player: &mut Player) {
    let head_pos = player.head().pos;
    if let Some(bonus) = self.bonuses.iter_mut().find(|b| b.pos == head_pos) {
      bonus.affect(player);
    }
    self.tick_lifetimes();
  }

  pub fn occupied_by(&self, pos: (i16, i16)) -> Option<&Effect> {
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
