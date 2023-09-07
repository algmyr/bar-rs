use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum Button {
  Left = 1,
  Middle = 2,
  Right = 3,
  ScrollUp = 4,
  ScrollDown = 5,
  // I think these are correct, I don't have my mouse to test.
  ScrollLeft = 6,
  ScrollRight = 7,
  Back = 8,
  Forward = 9,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputEvent {
  pub name: String,
  pub button: Button,
  pub modifiers: Vec<String>,
  pub x: i32,
  pub y: i32,
  pub relative_x: i32,
  pub relative_y: i32,
  pub output_x: i32,
  pub output_y: i32,
  pub width: i32,
  pub height: i32,
}
