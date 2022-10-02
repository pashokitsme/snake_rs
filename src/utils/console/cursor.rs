use std::io::stdout;
use crossterm::{execute, cursor};

pub fn to(column: u16, row: u16) {
  execute!(stdout(), cursor::MoveTo(column, row)).unwrap();
}