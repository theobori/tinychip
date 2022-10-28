use structopt::StructOpt;
use std::path::PathBuf;

use toychip::{
    emulator::EmulatorBuilder,
    apis::api::{
        ApiKind,
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
    api: Option<ApiKind>,
    /// Interpreter, value(s): original
    #[structopt(long)]
    interpreter: Option<InterpreterType>,
    /// Cycle(s) per second (Hz)
    #[structopt(long)]
    cycles: Option<u64>,
    /// use the original semantic for fx55, fx65
    #[structopt(long)]
    original_load: Option<bool>,
    /// use the original semantic for 8xy6, 8xye
    #[structopt(long)]
    original_shift: Option<bool>
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
    pub fn api(&self) -> ApiKind {
        self.api.unwrap_or_default()
    }

    /// Return the interpreter
    pub fn interpreter(&self) -> Box<dyn Interpreter> {
        self.interpreter.unwrap_or_default().into()
    }

    /// Return the interpreter
    pub fn cycles(&self) -> u64 {
        let ret = self.cycles.unwrap_or(500);

        if ret <= 500 {
            500
        } else if ret > 2000 {
            2000
        } else {
            ret
        }
    }

    /// Return original load state
    pub fn original_load(&self) -> bool {
        self.original_load.unwrap_or(false)
    }

    /// Return original shift state
    pub fn original_shift(&self) -> bool {
        self.original_shift.unwrap_or(false)
    }
}

fn main() -> Result<(), ChipError> {
    let args = Opt::from_args();

    let mut interpreter = args.interpreter();

    interpreter.set_original_load(args.original_load());
    interpreter.set_original_shift(args.original_shift());

    let mut emu = EmulatorBuilder::new()
        .set_api(args.api())
        .set_window_size(args.size())
        .set_window_title("toychip")
        .set_interpreter(interpreter)
        .set_clock(args.cycles())
        .build();

    emu.load_from_file(args.rom)?;
    emu.run();

    Ok(())
}
