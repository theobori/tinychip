use std::str::FromStr;

use crate::error::ChipError;
use crate::models::api::Api;
use crate::apis::libs::{
    sdl::SdlApi,
    sfml::SfmlApi
};

/// Public available implemented apis
#[derive(Debug, Clone, Copy)]
pub enum ApiKind {
    Sdl,
    Sfml
}

impl Default for ApiKind {
    fn default() -> Self {
        Self::Sdl
    }
}

impl FromStr for ApiKind {
    type Err = ChipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let api = match s {
            "sdl" => Self::Sdl,
            "sfml" => Self::Sfml,
            _ => {
                return Err(ChipError::UseApi);
            }
        };

        Ok(api)
    }
}

pub struct GraphicProp {
    /// ApiKind type
    pub api: ApiKind,
    /// Window title
    pub title: String,
    /// Window size
    pub size: (u32, u32),
}

// Window sizes
/// Minimum width
pub const WINDOW_MIN_W: u32 = 320;
/// Minimum height
pub const WINDOW_MIN_H: u32 = 160;
/// Maximum width
pub const WINDOW_MAX_W: u32 = 1280;
/// Maximum height
pub const WINDOW_MAX_H: u32 = 640;

// Rectangles number on x and y axis
/// X-axis
pub const RECTS_X: u32 = 64;
/// Y-axis
pub const RECTS_Y: u32 = 32;

impl From<GraphicProp> for Box<dyn Api> {
    fn from(prop: GraphicProp) -> Self {
        let (w, h) = prop.size;

        match prop.api {
            ApiKind::Sdl => Box::new(SdlApi::new(prop.title, w, h)),
            ApiKind::Sfml => Box::new(SfmlApi::new(prop.title, w, h))
        }
    }
}
