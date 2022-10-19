use chip8::{
    chip::Chip,
    graphics::api::GraphicApi
};

fn main() {
    Chip::new(GraphicApi::Sdl)
        .load_from_file("games/ibm_logo.ch8")
        .unwrap();
}
