use chip8::{
    chip::Emulator,
    graphics::api::GraphicApi
};

fn main() {
    Emulator::new(GraphicApi::Sdl)
        .load_from_file("games/ibm_logo.ch8")
        .unwrap();
}
