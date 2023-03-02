use self::{bonuses::EffectSpawner, player::Player, renderer::Renderer};
use rand::Rng;

mod bonuses;
mod player;
mod renderer;

pub struct InitSettings {
  pub field_size: (i16, i16),
  pub input_timeout: (u64, u64),
  pub parts_count: usize,
}

pub struct Game {
  player: Player,
  bonuses: EffectSpawner,
  renderer: Renderer,
  field_size: (i16, i16),
  running: bool,
}

impl Game {
  pub fn new(init: InitSettings) -> Game {
    let player = Player::new(&init);
    let renderer = Renderer::new(init.field_size);
    let bonuses = EffectSpawner::new();
    Game { player, bonuses, renderer, field_size: init.field_size, running: false }
  }

  pub fn start(&mut self) {
    self.running = true;
    self.render();
    loop {
      self.next();
      if !self.running {
        break;
      }
    }

    if self.player.is_lost {
      self.loss()
    }
  }

  fn loss(&mut self) {
    println!("\nТы проиграл.\nНажми что-нибудь для выхода.",);
    crossterm::event::read().unwrap();
  }

  fn tick(&mut self) {
    if self.bonuses.current_count() < 2 && rand::random::<f64>() < 0.05 {
      match rand::random::<f64>() {
        x if x > 0.95 => self.bonuses.remove(self.random_pos()),
        _ => self.bonuses.add(self.random_pos(), bonuses::EffectKind::Score(1)),
      }
    }

    self.player.tick();
    self.bonuses.tick(&mut self.player);

    if self.player.is_lost {
      self.running = false;
    }
  }

  fn next(&mut self) {
    self.tick();
    self.render();
  }

  fn render(&mut self) {
    self.renderer.render(&self.player, &self.bonuses)
  }

  fn random_pos(&self) -> (i16, i16) {
    loop {
      let pos = (rand::thread_rng().gen_range(2..self.field_size.0), rand::thread_rng().gen_range(3..self.field_size.1));
      if !self.is_occupied(pos) {
        return pos;
      }
    }
  }

  fn is_occupied(&self, pos: (i16, i16)) -> bool {
    self.player.is_occupied(pos).is_some() || self.bonuses.occupied_by(pos).is_some()
  }
}
