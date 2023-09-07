use sysinfo::SystemExt;

use crate::colors::Color;

use super::BlockInterface;

#[derive(Default)]
pub struct LoadBlock {
  text: String,
  color: Color,
  system: sysinfo::System,
}

impl BlockInterface for LoadBlock {
  fn name(&self) -> &str { "load" }
  fn color(&self) -> Color { self.color }
  fn text(&self) -> &str { &self.text }

  fn update(&mut self) -> anyhow::Result<()> {
    self.system.refresh_cpu();
    let load = self.system.load_average();
    self.text = format!("{:.2}", load.one);
    if load.one < 1.0 {
      self.color = Color::White;
    } else if load.one < 2.0 {
      self.color = Color::Yellow;
    } else {
      self.color = Color::Red;
      self.text += "!!!";
    }
    Ok(())
  }
}
