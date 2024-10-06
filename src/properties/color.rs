/// The unique Color model for this crate
#[derive(Clone, Copy)]
pub struct Color {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
    /// Alpha
    pub a: u8,
}

/// Pre-defined colors
pub enum ColorPreset {
    Red,
    Green,
    Blue,
    Black,
    White,
    Grey,
    Default,
    None,
}

/// Convert a pre-defined color into Color
impl Into<Color> for ColorPreset {
    fn into(self) -> Color {
        match self {
            ColorPreset::Red => Color::from((255, 0, 0)),
            ColorPreset::Green => Color::from((0, 255, 0)),
            ColorPreset::Blue => Color::from((0, 0, 255)),
            ColorPreset::Black => Color::from((0, 0, 0)),
            ColorPreset::White => Color::from((255, 255, 255)),
            ColorPreset::Grey => Color::from((127, 127, 127)),
            ColorPreset::Default => Color::from((0, 0, 0)),
            ColorPreset::None => Color::from((0, 0, 0, 0)),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        ColorPreset::Default.into()
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self { r, g, b, a }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b, a: 255 }
    }
}
