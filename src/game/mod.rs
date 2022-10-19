use self::{player::Player, renderer::Renderer, bonuses::BonusProvider};
use rand::Rng;

mod bonuses;
mod renderer;
mod player;
mod controls;

const POSSIBLE_INSULTS: [&str; 6] = ["нуп", "интернет-нубасик", "дэб", "дурак", "дуралей", "научись играть"];

pub struct Game {
  player: Player,
  bonuses: BonusProvider,
  renderer: Renderer,
  field_size: (i16, i16),
  running: bool,
}

impl Game {
  pub fn new(size: (i16, i16), init_parts: usize) -> Game {
    let player = Player::new(size, init_parts);
    let renderer = Renderer::new(size);
    let bonuses = BonusProvider::new();
    Game { player, bonuses, renderer, field_size: size, running: false }
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
    println!("Ты проиграл, {}. Нажми что-нибудь для выхода.", POSSIBLE_INSULTS[rand::thread_rng().gen_range(0..POSSIBLE_INSULTS.len())]);
    crossterm::event::read().unwrap();
  }

  fn tick(&mut self) {
    if self.bonuses.current_count() < 2 && rand::random::<f64>() < 0.05 {
      match rand::random::<f64>() {
        x if x > 0.95 => self.bonuses.place_removepart(self.random_pos()),
        _ => self.bonuses.place_addpart(self.random_pos())
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
    self.player.is_occupied(pos).is_some() || self.bonuses.is_occupied(pos).is_some()
  }
}