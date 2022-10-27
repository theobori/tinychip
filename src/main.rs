use structopt::StructOpt;
use std::path::PathBuf;

use toychip::{
    emulator::EmulatorBuilder,
    graphics::api::{
        Api,
        RECTS_X,
        RECTS_Y
    },
    interpreters::types::InterpreterType,
    models::{
        core::Core,
        interpreter::Interpreter
    },
    error::ChipError
};

#[derive(StructOpt, Debug)]
#[structopt(name = "chip8")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    rom: PathBuf,
    /// Window width
    #[structopt(short, long)]
    width: Option<u32>,
    /// Window height
    #[structopt(short, long)]
    height: Option<u32>,
    /// Graphical API, value(s): sfml, sdl
    #[structopt(long)]
    api: Option<Api>,
    /// Interpreter, value(s): original
    #[structopt(long)]
    interpreter: Option<InterpreterType>,
    /// Cycle(s) per second (Hz)
    #[structopt(long)]
    cycles: Option<usize>
}

impl Opt {
    /// Return the window width
    pub fn width(&self) -> u32 {
        self.width.unwrap_or(RECTS_X * 20)
    }

    /// Return the window height
    pub fn height(&self) -> u32 {
        self.height.unwrap_or(RECTS_Y * 20)
    }

    /// Return the window size
    pub fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    /// Return the graphical api
    pub fn api(&self) -> Api {
        self.api.unwrap_or_default()
    }

    /// Return the interpreter
    pub fn interpreter(&self) -> Box<dyn Interpreter> {
        self.interpreter.unwrap_or_default().into()
    }

    /// Return the interpreter
    pub fn cycles(&self) -> usize {
        self.cycles.unwrap_or(500)
    }
}

fn main() -> Result<(), ChipError> {
    let args = Opt::from_args();

    let mut emu = EmulatorBuilder::new()
        .set_api(args.api())
        .set_window_size(args.size())
        .set_window_title("chip8 emulator")
        .set_interpreter(args.interpreter())
        .build();

    emu.load_from_file(args.rom)?;
    emu.run();

    Ok(())
}
