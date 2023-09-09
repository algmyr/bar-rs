use std::sync::{Arc, Mutex};
use std::time::Duration;

use mpris::{PlaybackStatus, Player};

use super::{make_block, BlockInterface, BlockOutput};
use crate::colors::Color;
use crate::events::InputEvent;

pub struct MediaBlock {
  blocks: Vec<BlockOutput>,
  color: Color,
  player_finder: Arc<Mutex<mpris::PlayerFinder>>,
}

impl Default for MediaBlock {
  fn default() -> Self {
    Self {
      blocks: Default::default(),
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
  let cmp_key = |player: &Player| {
    let name_priority = match player.bus_name_player_name_part() {
      "spotify" => 0,
      "kdeconnect" => 2,
      _ => 1,
    };

    let status_priority = match player.get_playback_status() {
      Ok(PlaybackStatus::Playing) => 0,
      Ok(PlaybackStatus::Paused) => 1,
      Ok(PlaybackStatus::Stopped) => 2,
      _ => 3,
    };

    return (status_priority, name_priority);
  };

  if let Ok(mut players) = player_finder.find_all() {
    players.sort_by_cached_key(cmp_key);
    return players.into_iter().next();
  }
  None
}

impl BlockInterface for MediaBlock {
  fn name(&self) -> &str { "media" }

  fn update(&mut self) -> anyhow::Result<()> {
    if let Some(player) = find_active_non_kdeconnect(&self.player_finder.lock().unwrap())
    {
      if let Ok(metadata) = player.get_metadata() {
        let artist = metadata.artists().map(|a| a.join(", "));
        let album = metadata.album_name();
        let title = metadata.title();
        let status = match player.get_playback_status() {
          Ok(PlaybackStatus::Playing) => "▶ ",
          Ok(PlaybackStatus::Paused) => "⏸ ",
          Ok(PlaybackStatus::Stopped) => "⏹ ",
          _ => "⏹ ",
        };

        let progress = format!(
          "{}/{}",
          format_duration(player.get_position().ok()),
          format_duration(metadata.length())
        );

        let mut blocks = vec![];
        blocks.push(make_block("media_status", &status, Color::White));
        blocks.push(make_block("media_title", &format!("{}  ", title.unwrap_or("Unknown Title")), Color::Red));
        if let Some(artist) = artist {
          blocks.push(make_block("media_artist", &format!("{}  ", artist), Color::Yellow));
        }
        if let Some(album) = album {
          blocks.push(make_block("media_album", &format!("{}  ", album), Color::Blue));
        }
        blocks.push(make_block("media_progress", &progress, Color::Green));
        self.blocks = blocks;
        return Ok(());
      }
      self.blocks = vec![make_block(
        "media_error",
        "No song metadata found",
        self.color,
      )];
      return Ok(());
    }
    self.blocks = vec![make_block("media_error", "No player found", self.color)];
    Ok(())
  }

  fn get_blocks(&self) -> Vec<BlockOutput> { self.blocks.clone() }

  fn handle_input(&mut self, event: &InputEvent) -> anyhow::Result<bool> {
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
      Back => dev().and_then(|player| player.previous().ok()),
      Forward => dev().and_then(|player| player.next().ok()),
      _ => return Ok(false),
    };
    Ok(true)
  }
}
