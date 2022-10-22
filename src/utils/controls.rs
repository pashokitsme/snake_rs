use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode};

pub fn get_input(timeout: u64) -> Option<(i16, i16)> {
  if !poll(Duration::from_millis(timeout)).unwrap() {
    return None;
  }

  let ev = read().unwrap();
  if ev == get_key('d') || ev == get_key('в') {
    return Some((1, 0));
  }

  if ev == get_key('s') || ev == get_key('ы') {
    return Some((0, 1));
  }

  if ev == get_key('a') || ev == get_key('ф') {
    return Some((-1, 0));
  }

  if ev == get_key('w') || ev == get_key('ц') {
    return Some((0, -1));
  }

  None
}

fn get_key(ch: char) -> Event {
  Event::Key(KeyCode::Char(ch).into())
}