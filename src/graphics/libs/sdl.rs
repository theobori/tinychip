use crate::{
    models::graphic::Graphic,
    event::{
        Hotkey,
        Mouse
    },
    properties::{
        rectangle::Rectangle,
        color::Color
    }
};

pub struct SdlGraphic {

}

impl SdlGraphic {

}

impl Graphic for SdlGraphic {
    fn init(&mut self, title: String, w: u32, h: u32) {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn hotkeys(&self) -> Vec<Hotkey> {
        todo!()
    }

    fn mouse(&self) -> Vec<Mouse> {
        todo!()
    }

    fn draw_rect(&self, rect: Rectangle, color: Color) {
        todo!()
    }

    fn is_window_open(&self) -> bool {
        todo!()
    }
}
