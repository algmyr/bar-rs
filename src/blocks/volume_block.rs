use std::sync::{Arc, Mutex};

use libpulse_binding::volume::VolumeLinear;
use pulsectl::controllers::{DeviceControl, SinkController, AppControl};

use super::BlockInterface;
use crate::{colors::Color, events::InputEvent};

pub struct VolumeBlock {
  text: String,
  debug: String,
  handler: Arc<Mutex<SinkController>>,
}

unsafe impl Send for VolumeBlock {}

impl Default for VolumeBlock {
  fn default() -> Self {
    let handler = Arc::new(Mutex::new(SinkController::create().unwrap()));
    Self { text: Default::default(), debug: Default::default(), handler }
  }
}

impl VolumeBlock {
  fn get_default_device(&self) -> pulsectl::controllers::types::DeviceInfo {
    self.handler.lock().unwrap().get_default_device().expect("Could not get list of playback devices.")
  }
}

impl BlockInterface for VolumeBlock {
  fn name(&self) -> &str { "volume" }
  fn color(&self) -> Color { Color::Red }
  fn text(&self) -> &str { &self.text }

  fn update(&mut self) {
    let dev = self.get_default_device();
    self.text = dev.volume.avg().to_string().trim().to_string();
  }

  fn handle_input(&mut self, event: &InputEvent) {
    let handler = &mut self.handler.lock().unwrap();
    let dev = handler.get_default_device().expect("Could not get list of playback devices.");

    // I fucking hate this library.
    use crate::events::Button::*;
    match event.button {
      Left => handler.set_device_mute_by_index(dev.index, !dev.mute),
      Middle => (),
      Right => (),
      ScrollUp => handler.increase_device_volume_by_percent(dev.index, 0.01),
      ScrollDown => handler.decrease_device_volume_by_percent(dev.index, 0.01),
      ScrollLeft => (),
      ScrollRight => (),
      Back =>  (),
      Forward =>  (),
    }
  }
}
