use chip8::{
    emulator::EmulatorBuilder,
    graphics::api::{
        Api,
        GraphicProp,
        RECT_W,
        RECT_H
    },
    interpreters::interpreter::ChipInterpreter,
    models::core::Core, error::ChipError
};

fn main() -> Result<(), ChipError> {
    let mut emu = EmulatorBuilder::new()
        .set_api(Api::Sdl)
        .set_window_size((RECT_W * 20, RECT_H * 20))
        .set_window_title("chip8")
        .set_interpreter(Box::new(ChipInterpreter::new()))
        .build();

    emu.load_from_file("games/ibm_logo.ch8")?;
    emu.run();

    Ok(())
}
