use chrono::Datelike;

use super::BlockInterface;
use crate::colors::Color;

#[derive(Default)]
pub struct DateBlock {
  text: String,
}

impl BlockInterface for DateBlock {
  fn name(&self) -> &str { "date" }
  fn color(&self) -> Color { Color::White }
  fn text(&self) -> &str { &self.text }

  fn update(&mut self) {
    let now = chrono::Local::now();
    let suffix = match now.day() {
      1 | 21 | 31 => "st",
      2 | 22 => "nd",
      3 | 23 => "rd",
      _ => "th",
    };
    let fmt = format!("%A %-d{suffix} of %B");
    self.text = now.format(&fmt).to_string();
  }
}
