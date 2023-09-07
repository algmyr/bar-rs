use std::{time::{Duration, Instant}, collections::VecDeque};

use sysinfo::{SystemExt, NetworkExt};

use crate::{colors::Color, blocks::make_block};

use super::BlockInterface;

const HIST_LEN: Duration = Duration::from_secs(10);

pub struct NetworkBlock {
  system: sysinfo::System,
  up_history: VecDeque<(Instant, u64)>,
  down_history: VecDeque<(Instant, u64)>,
  bps_up: Option<u64>,
  bps_down: Option<u64>,
}

impl Default for NetworkBlock {
  fn default() -> Self {
    Self {
      system: sysinfo::System::new_all(),
      up_history: Default::default(),
      down_history: Default::default(),
      bps_up: None,
      bps_down: None,
    }
  }
}

impl BlockInterface for NetworkBlock {
  fn name(&self) -> &str { "network" }

  fn update(&mut self) -> anyhow::Result<()> {
    let update = |history: &mut VecDeque<(Instant, u64)>, time, value| {
      history.push_back((time, value));
      while history
        .front()
        .map(|(t, _)| time - *t > HIST_LEN)
        .unwrap_or(false)
      {
        history.pop_front();
      }

      let (old_time, old_value) = history.front().unwrap();
      let bytes = value - old_value;
      let ms = (time - *old_time).as_millis() as u64;

      if ms == 0 {
        None
      } else {
        Some(bytes * 1000 / ms)
      }
    };

    self.system.refresh_networks();

    let mut up = 0;
    let mut down = 0;
    for (name, data) in self.system.networks() {
      if name.starts_with("enp") {
        up += data.total_received();
        down += data.total_transmitted();
      }
    }

    let now = std::time::Instant::now();
    self.bps_up = update(&mut self.up_history, now, up);
    self.bps_down = update(&mut self.down_history, now, down);

    Ok(())
  }

  fn to_string(&self) -> String {
    let pretty_speed = |bps: Option<u64>| {
      if let Some(bps) = bps {
        if bps >= 100000000 {
          return format!("{:4}MB", bps / 1000000);
        }
        if bps >= 10000000 {
          return format!("{:2}.{:01}MB", bps / 1000000, bps / 100000 % 10);
        }
        if bps >= 1000000 {
          return format!("{:1}.{:02}MB", bps / 1000000, bps / 10000 % 100);
        }
        if bps >= 100000 {
          return format!("{:4}kB", bps / 1000);
        }
        if bps >= 10000 {
          return format!("{:2}.{:01}kB", bps / 1000, bps / 100 % 10);
        }
        if bps >= 1000 {
          return format!("{:1}.{:02}kB", bps / 1000, bps / 10 % 100);
        }
        return format!("{:4} B", bps);
      }
      "N/A".to_string()
    };

    let speeds = format!(
      "▲ {} ▼ {}",
      pretty_speed(self.bps_up),
      pretty_speed(self.bps_down)
    );
    format!(
      "{},{}",
      make_block("net_device", "Eth ", Color::White),
      make_block("net_speed", &speeds, Color::Blue),
    )
  }
}
