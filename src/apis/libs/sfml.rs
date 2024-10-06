use crate::{
    event::Input,
    models::{api::Api, audio::Audio},
    properties::{color::Color, rectangle::Rectangle},
};

pub struct SfmlApi {}

impl SfmlApi {
    pub fn new(_title: String, _w: u32, _h: u32) -> Self {
        todo!()
    }
}

impl Api for SfmlApi {
    fn clear(&mut self) {
        todo!()
    }

    fn draw_rect(&mut self, _rect: Rectangle, _color: Color) {
        todo!()
    }

    fn is_window_open(&self) -> bool {
        todo!()
    }

    fn display(&mut self) {
        todo!()
    }

    fn events(&mut self) -> Vec<Input> {
        todo!()
    }

    fn window_size(&self) -> (u32, u32) {
        todo!()
    }
}

impl Audio for SfmlApi {
    fn resume_beep(&mut self) {
        todo!()
    }

    fn pause_beep(&mut self) {
        todo!()
    }
}
