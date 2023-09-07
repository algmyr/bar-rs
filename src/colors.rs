#[derive(Debug, Clone, Copy)]
pub enum Color {
  Gray,
  Black,
  LightBlack,
  White,
  LightWhite,
  Red,
  LightRed,
  Green,
  LightGreen,
  Yellow,
  LightYellow,
  Blue,
  LightBlue,
  Magenta,
  LightMagenta,
  Cyan,
  LightCyan,
}

impl Color {
  pub fn value(&self) -> &str {
    match self {
      Color::Gray => "#9C998E",
      Color::Black => "#303030",
      Color::LightBlack => "#767676",
      Color::White => "#cccccc",
      Color::LightWhite => "#FFFFFF",
      Color::Red => "#D65453",
      Color::LightRed => "#FF7777",
      Color::Green => "#48A16F",
      Color::LightGreen => "#80D9A5",
      Color::Yellow => "#C09C4F",
      Color::LightYellow => "#F7E285",
      Color::Blue => "#6780BD",
      Color::LightBlue => "#9DADD0",
      Color::Magenta => "#DF679A",
      Color::LightMagenta => "#FF9FD4",
      Color::Cyan => "#4E9E9E",
      Color::LightCyan => "#85D5D4",
    }
  }
}

impl Default for Color {
  fn default() -> Self { Color::White }
}
