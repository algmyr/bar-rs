use serde::{Deserialize, Serialize};

use crate::colors::Color;
use crate::events::InputEvent;

mod clock_block;
mod date_block;
mod load_block;
mod media_block;
mod network_block;
mod separator;
mod volume_block;

pub use clock_block::ClockBlock;
pub use date_block::DateBlock;
pub use load_block::LoadBlock;
pub use media_block::MediaBlock;
pub use network_block::NetworkBlock;
pub use separator::Separator;
pub use volume_block::VolumeBlock;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BlockOutput {
  name: String,
  full_text: String,
  color: String,
  separator: bool,
  separator_block_width: u32,
}

fn escape(s: &str) -> String {
  let mut res = String::new();
  for c in s.chars() {
    if c == '"' {
      res.push_str("&quot;");
    } else {
      res.push(c);
    }
  }
  res
}

fn make_block(name: &str, text: &str, color: Color) -> BlockOutput {
  BlockOutput {
    name: escape(name),
    full_text: escape(text),
    color: escape(color.value()),
    separator: false,
    separator_block_width: 0,
  }
}

pub trait BlockInterface {
  fn name(&self) -> &str;
  fn color(&self) -> Color { Color::default() }
  fn text(&self) -> &str { "" }

  fn get_blocks(&self) -> Vec<BlockOutput> {
    vec![make_block(self.name(), self.text(), self.color())]
  }

  fn update(&mut self) -> anyhow::Result<()> { Ok(()) }
  fn handle_input(&mut self, _event: &InputEvent) -> anyhow::Result<bool> { Ok(false) }
}
