pub mod clear {
  use std::io::stdout;
  use crossterm::terminal::{ Clear, ClearType };
  use crossterm::execute;

  pub fn under_cursor() {
    execute!(stdout(), Clear(ClearType::FromCursorDown)).unwrap()
  }

  pub fn all() {
    execute!(stdout(), Clear(ClearType::All)).unwrap()
  }
}

pub mod cursor {
  use std::io::stdout;
  use crossterm::{execute, cursor};

  pub fn to(column: u16, row: u16) {
    execute!(stdout(), cursor::MoveTo(column, row)).unwrap();
  }
}