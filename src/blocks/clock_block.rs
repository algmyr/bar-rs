use crate::colors::Color;

use super::BlockInterface;

#[derive(Default)]
pub struct ClockBlock {
  text: String,
}

impl BlockInterface for ClockBlock {
  fn name(&self) -> &str { "clock" }
  fn color(&self) -> Color { Color::White }
  fn text(&self) -> &str { &self.text }

  fn update(&mut self) { self.text = chrono::Local::now().format("%H:%M").to_string(); }
}
