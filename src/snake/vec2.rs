#[derive(Clone, Copy)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T
}

impl<T> Vec2<T> {
  pub fn new(x: T, y: T) -> Vec2<T> {
    Vec2 { x, y }
  }
}