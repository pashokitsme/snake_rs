use std::io::stdout;
use crossterm::terminal::{ Clear, ClearType };
use crossterm::execute;

pub fn above_cursor() {
  execute!(stdout(), Clear(ClearType::FromCursorUp)).unwrap()
}

pub fn all() {
  execute!(stdout(), Clear(ClearType::All)).unwrap()
}