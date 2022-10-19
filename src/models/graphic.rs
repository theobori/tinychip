use crate::properties::{
    rectangle::Rectangle,
    color::Color,
};

use crate::event::{
    Hotkey,
    Mouse
};

pub trait Graphic {
    /// Setup the graphic API
    fn init(&mut self, title: String, w: u32, h: u32);
    /// Clear the screen
    fn clear(&mut self);
    /// Get the pressed hotkeys in the `Hotkeys` type
    fn hotkeys(&self) -> Vec<Hotkey>;
    /// Get the pressed mouse buttons with the mouse coordinates
    fn mouse(&self) -> Vec<Mouse>;
    /// Draw a rectangle at the screen
    fn draw_rect(&self, rect: Rectangle, color: Color);
    /// Return the window state
    fn is_window_open(&self) -> bool;
}
