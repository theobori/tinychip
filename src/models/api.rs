use crate::properties::{color::Color, rectangle::Rectangle};

use crate::event::Input;

use super::audio::Audio;

pub trait Api: Audio {
    /// Clear the screen
    fn clear(&mut self);
    /// Draw a rectangle at the screen
    fn draw_rect(&mut self, rect: Rectangle, color: Color);
    /// Return the window state
    fn is_window_open(&self) -> bool;
    /// Display the drawing on the screen as a complete picture
    fn display(&mut self);
    /// Return the pressed inputs
    fn events(&mut self) -> Vec<Input>;
    /// Return the window size (width, height)
    fn window_size(&self) -> (u32, u32);
}
