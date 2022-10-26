use chip8::{
    emulator::EmulatorBuilder,
    graphics::api::{
        Api,
        RECTS_X,
        RECTS_Y
    },
    interpreters::interpreter::ChipInterpreter,
    models::core::Core, error::ChipError
};

fn main() -> Result<(), ChipError> {
    let mut emu = EmulatorBuilder::new()
        .set_api(Api::Sdl)
        .set_window_size((RECTS_X * 20, RECTS_Y * 20))
        .set_window_title("chip8")
        .set_interpreter(Box::new(ChipInterpreter::new()))
        .build();

    emu.load_from_file("games/test_opcode.ch8")?;
    emu.run();

    Ok(())
}
