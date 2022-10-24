use chip8::{
    emulator::Emulator,
    graphics::api::GraphicType, models::core::Core
};

fn main() {
    let api = GraphicType::Sdl(
        String::from("title"),
        800,
        600
    );

    let mut emulator = Emulator::new(api);
    
    emulator.load_from_file("games/ibm_logo.ch8").unwrap();
    emulator.init();
    emulator.run();
}
