/// Represent a rectangle (position + size)
#[derive(Debug)]
pub struct Rectangle {
    /// x-axis
    pub x: i32,
    /// y-axis
    pub y: i32,
    /// Width
    pub w: u32,
    /// Height
    pub h: u32,
}

impl From<(i32, i32, u32, u32)> for Rectangle {
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> Self {
        Self { x, y, w, h }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        (0, 0, 0, 0).into()
    }
}
