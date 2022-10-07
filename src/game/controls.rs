use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode};

pub fn get_input() -> Option<(i16, i16)> {
  if poll(Duration::from_millis(1000)).unwrap() == false {
    return None;
  }

  let ev = read().unwrap();
  if ev == Event::Key(KeyCode::Char('d').into()) {
    return Some((1, 0));
  }

  if ev == Event::Key(KeyCode::Char('s').into()) {
    return Some((0, 1));
  }

  if ev == Event::Key(KeyCode::Char('a').into()) {
    return Some((-1, 0));
  }

  if ev == Event::Key(KeyCode::Char('w').into()) {
    return Some((0, -1));
  }

  None
}