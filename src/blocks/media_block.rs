use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::{make_block, BlockInterface};
use crate::colors::Color;
use crate::events::InputEvent;

pub struct MediaBlock {
  text: String,
  color: Color,
  player_finder: Arc<Mutex<mpris::PlayerFinder>>,
}

impl Default for MediaBlock {
  fn default() -> Self {
    Self {
      text: Default::default(),
      color: Default::default(),
      player_finder: Arc::new(Mutex::new(mpris::PlayerFinder::new().unwrap())),
    }
  }
}

unsafe impl Send for MediaBlock {}

fn format_duration(dur: Option<Duration>) -> String {
  if let Some(dur) = dur {
    let seconds = dur.as_secs();
    if seconds > 3600 {
      format!(
        "{}:{:02}:{:02}",
        seconds / 3600,
        seconds / 60 % 60,
        seconds % 60
      )
    } else {
      format!("{:02}:{:02}", seconds / 60, seconds % 60)
    }
  } else {
    "?:??".to_string()
  }
}

fn find_active_non_kdeconnect(
  player_finder: &mpris::PlayerFinder,
) -> Option<mpris::Player> {
  let mut players = player_finder.find_all().ok()?;
  players.retain(|player| player.bus_name_player_name_part() != "kdeconnect");
  players.into_iter().next()
}

impl BlockInterface for MediaBlock {
  fn name(&self) -> &str { "media" }

  fn update(&mut self) {
    if let Some(player) = find_active_non_kdeconnect(&self.player_finder.lock().unwrap())
    {
      if let Ok(metadata) = player.get_metadata() {
        let artist = if let Some(artists) = metadata.artists() {
          artists.join(", ")
        } else {
          "Unknown Artist".to_string()
        };
        let album = metadata.album_name().unwrap_or("Unknown Album");
        let title = metadata.title().unwrap_or("Unknown Title");
        let status = match player.get_playback_status() {
          Ok(mpris::PlaybackStatus::Playing) => "▶ ",
          Ok(mpris::PlaybackStatus::Paused) => "⏸ ",
          Ok(mpris::PlaybackStatus::Stopped) => "⏹ ",
          _ => "⏹ ",
        };

        let progress = format!(
          "{}/{}",
          format_duration(player.get_position().ok()),
          format_duration(metadata.length())
        );

        self.text = format!(
          "{}, {}, {}, {}, {}",
          // status
          make_block("media_status", &status, Color::White),
          make_block("media_title", &format!("{title}  "), Color::Red),
          make_block("media_artist", &format!("{artist}  "), Color::Yellow),
          make_block("media_album", &format!("{album}  "), Color::Blue),
          make_block("media_progress", &progress, Color::Green),
          // progress
        );
        return;
      }
      self.text = make_block("media_error", "No song metadata found", self.color);
      return;
    }
    self.text = make_block("media_error", "No player found", self.color);
  }

  fn to_string(&self) -> String { self.text.clone() }

  fn handle_input(&mut self, event: &InputEvent) {
    let dev = || find_active_non_kdeconnect(&self.player_finder.lock().unwrap());
    use crate::events::Button::*;
    match event.button {
      Left => dev().and_then(|player| player.play_pause().ok()),
      Middle => dev().and_then(|player| player.previous().ok()),
      Right => dev().and_then(|player| player.next().ok()),
      ScrollUp => dev().and_then(|player| {
        let volume = player.get_volume().ok()?;
        player.set_volume(volume.max(0.01) * 1.05).ok()
      }),
      ScrollDown => dev().and_then(|player| {
        let volume = player.get_volume().ok()?;
        player.set_volume(volume / 1.05).ok()
      }),
      ScrollLeft => None,
      ScrollRight => None,
      Back => dev().and_then(|player| player.previous().ok()),
      Forward => dev().and_then(|player| player.next().ok()),
    };
    self.update();
  }
}
