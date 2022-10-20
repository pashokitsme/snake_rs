pub mod console;
pub mod controls;

pub fn wrapped_pos(bounds: (i16, i16), desired: (i16, i16)) -> (i16, i16) {
  let mut result = desired; 
  match desired {
    _ if desired.0 >= bounds.0 => result.0 -= bounds.0 - 2,
    _ if desired.0 <= 1 => result.0 = bounds.0 - 1,
    (_, _) => {}
  }
  match desired {
    _ if desired.1 >= bounds.1 => result.1 = 2,
    _ if desired.1 <= 1 => result.1 = bounds.1 - 1,
    (_, _) => {}
  }

  result
}