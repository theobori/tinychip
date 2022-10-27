use crate::{
    models::graphic::Graphic,
    event::{
        Input
    },
    properties::{
        rectangle::Rectangle,
        color::Color
    }
};

pub struct SfmlGraphic {

}

impl SfmlGraphic {
    pub fn new(_title: String, _w: u32, _h: u32) -> Self {
        todo!()
    }
}

impl Graphic for SfmlGraphic {
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
