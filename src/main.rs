mod renderer;
mod snake;
mod utils;

fn main() {
  let renderer = renderer::new(snake::Vec2::new(16, 16));
  renderer.render();
  println!("Heyo!");
}
