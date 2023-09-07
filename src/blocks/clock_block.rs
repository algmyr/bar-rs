use super::BlockInterface;
use crate::colors::Color;

#[derive(Default)]
pub struct ClockBlock {
  text: String,
}

impl BlockInterface for ClockBlock {
  fn name(&self) -> &str { "clock" }
  fn color(&self) -> Color { Color::White }
  fn text(&self) -> &str { &self.text }

  fn update(&mut self) -> anyhow::Result<()> {
    self.text = chrono::Local::now().format("%H:%M").to_string();
    Ok(())
  }
}
