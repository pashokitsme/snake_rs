pub mod clear {
  use crossterm::execute;
  use crossterm::terminal::{Clear, ClearType};
  use std::io::stdout;

  pub fn upper_cursor() {
    execute!(stdout(), Clear(ClearType::FromCursorUp)).unwrap()
  }

  pub fn all() {
    execute!(stdout(), Clear(ClearType::All)).unwrap()
  }
}

pub mod cursor {
  use crossterm::{cursor, execute};
  use std::io::stdout;

  pub fn to(column: u16, row: u16) {
    execute!(stdout(), cursor::MoveTo(column, row)).unwrap();
  }
}
