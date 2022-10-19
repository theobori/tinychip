use crate::models::graphic::Graphic;

/// Public available implemented apis
pub enum GraphicApi {
    Sdl,
    Sfml
}

impl From<GraphicApi> for Box<dyn Graphic> {
    fn from(api: GraphicApi) -> Self {
        match api {
            GraphicApi::Sdl => todo!(),
            GraphicApi::Sfml => todo!(),
        }
    }
}
