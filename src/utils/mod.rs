pub mod console;

pub fn wrapped_pos(bounds: (i16, i16), desired: (i16, i16)) -> (i16, i16) {
  let mut result = desired; 
  match desired {
    _ if desired.0 >= bounds.0 => result.0 -= bounds.0 - 1,
    _ if desired.0 <= 0 => result.0 = bounds.0 - 2,
    _ if desired.1 >= bounds.1 => result.1 = 1,
    _ if desired.1 <= 0 => result.1 = bounds.0 - 2,
    (_, _) => unreachable!()
  }
  
  result
}