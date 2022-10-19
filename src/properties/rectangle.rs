/// Represent a rectangle (position + size)
pub struct Rectangle {
    /// x-axis
    x: u32,
    /// y-axis
    y: u32,
    /// Width
    w: u32,
    /// Height
    h: u32
}

impl From<(u32, u32, u32, u32)> for Rectangle {
    fn from((x, y, w, h): (u32, u32, u32, u32)) -> Self {
        Self {x, y, w, h}
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        (0, 0, 0, 0).into()
    }
}