use crate::models::graphic::Graphic;
use crate::graphics::libs::{
    sdl::SdlGraphic,
    sfml::SfmlGraphic
};

/// Public available implemented apis
#[derive(Clone)]
pub enum GraphicType {
    Sdl(String, u32, u32),
    Sfml(String, u32, u32)
}

impl From<GraphicType> for Box<dyn Graphic> {
    fn from(api: GraphicType) -> Self {
        match api {
            GraphicType::Sdl(title, w, h) => Box::new(SdlGraphic::new(title, w , h)),
            GraphicType::Sfml(title, w , h) => todo!(),
        }
    }
}
