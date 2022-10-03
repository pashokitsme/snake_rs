mod renderer;
mod snake;
mod utils;

fn main() {
  let player = snake::player::Player::new();
  let renderer = renderer::new(snake::Vec2::new(16, 16));
  renderer.render();
  println!("Heyo!");
}
