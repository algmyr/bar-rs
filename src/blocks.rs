use crate::{colors::Color, events::InputEvent};

mod separator;
mod clock_block;
mod date_block;
mod load_block;
mod network_block;
mod media_block;
mod volume_block;

pub use separator::Separator;
pub use clock_block::ClockBlock;
pub use date_block::DateBlock;
pub use load_block::LoadBlock;
pub use network_block::NetworkBlock;
pub use media_block::MediaBlock;
pub use volume_block::VolumeBlock;

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

fn make_block(name: &str, text: &str, color: Color) -> String {
  format!(
    r#"{{ "name": "{}", "full_text": "{}", "color": "{}", "separator": false, "separator_block_width": 0 }}"#,
    escape(name),
    escape(text),
    escape(color.value())
  )
}

pub trait BlockInterface {
  fn name(&self) -> &str;
  fn color(&self) -> Color { Color::default() }
  fn text(&self) -> &str { "" }

  fn to_string(&self) -> String { make_block(self.name(), self.text(), self.color()) }

  fn update(&mut self) -> anyhow::Result<()> { Ok(()) }
  fn handle_input(&mut self, _event: &InputEvent) -> anyhow::Result<bool> { Ok(false) }
}
