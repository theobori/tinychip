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

}

impl Graphic for SfmlGraphic {
    fn clear(&mut self) {
        todo!()
    }

    fn draw_rect(&mut self, rect: Rectangle, color: Color) {
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
}
