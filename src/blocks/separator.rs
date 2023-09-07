use crate::colors::Color;

use super::BlockInterface;

#[derive(Default)]
pub struct Separator {}

impl BlockInterface for Separator {
  fn name(&self) -> &str { "separator" }
  fn color(&self) -> Color { Color::Gray }
  fn text(&self) -> &str { " ‹ " }

  fn update(&mut self) {}
}
