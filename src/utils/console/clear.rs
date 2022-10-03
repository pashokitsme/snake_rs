use std::io::stdout;
use crossterm::terminal::{ Clear, ClearType };
use crossterm::execute;

pub fn under_cursor() {
  execute!(stdout(), Clear(ClearType::FromCursorDown)).unwrap()
}

pub fn all() {
  execute!(stdout(), Clear(ClearType::All)).unwrap()
}