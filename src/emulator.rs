use std::{
    fs::File,
    path::Path,
    io::Read,
    thread,
    time
};

use crate::interpreters::interpreter::ChipInterpreter;
use crate::models::{
    core::Core,
    api::Api,
    interpreter::Interpreter
};
use crate::apis::api::{
    GraphicProp,
    ApiKind,
    WINDOW_MIN_W,
    WINDOW_MIN_H
};
use crate::error::ChipError;
use crate::properties::{
    color::ColorPreset,
    rectangle::Rectangle
};

impl Default for EmulatorBuilder {
    fn default() -> Self {
        Self {
            api_prop: GraphicProp {
                api: ApiKind::Sdl,
                title: String::from("chip8"),
                size: (WINDOW_MIN_W, WINDOW_MIN_H)
            },
            interpreter: Box::new(ChipInterpreter::new()),
            clock: 500
        }
    }
}

/// Build a `Emulator` struct
pub struct EmulatorBuilder {
    api_prop: GraphicProp,
    interpreter: Box<dyn Interpreter>,
    clock: u64
}

impl EmulatorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the api type
    pub fn set_api(mut self, api_type: ApiKind) -> Self {
        self.api_prop.api = api_type;

        self
    }

    /// Set the window title
    pub fn set_window_title<S: Into<String>>(mut self, title: S) -> Self {
        self.api_prop.title = title.into();

        self
    }

    /// Set the window size
    pub fn set_window_size(mut self, size: (u32, u32)) -> Self {
        self.api_prop.size = size;

        self
    }

    /// Set the interpreter
    pub fn set_interpreter(
        mut self,
        interpreter: Box<dyn Interpreter>
    ) -> Self {
        self.interpreter = interpreter;

        self
    }

    /// Set the clock
    pub fn set_clock(mut self, clock: u64) -> Self {
        self.clock = clock;

        self
    }

    /// Build the emulator
    pub fn build(self) -> Emulator {
        Emulator {
            api: self.api_prop.into(),
            interpreter: self.interpreter,
            clock: self.clock
        }
    }
}

/// Main structure that will contains almost everything
pub struct Emulator {
    /// Chip8 interpreter
    interpreter: Box<dyn Interpreter>,
    /// Graphical API
    api: Box<dyn Api>,
    /// Cycles per second (hz)
    pub clock: u64
}

impl Emulator {
    /// Load program raw bytes
    pub fn load<T: Into<Vec<u8>>>(&mut self, program: T) {
        self.interpreter.load_program(program.into());
    }

    /// Load a program from file
    pub fn load_from_file<P: AsRef<Path>>(
        &mut self, path: P
    ) -> Result<(), ChipError> {
        let f = File::open(path);
        
        match f {
            Ok(mut file) => {
                let mut data = Vec::<u8>::new();

                match file.read_to_end(data.as_mut()) {
                    Ok(_) => {
                        self.load(data);

                        Ok(())
                    }
                    Err(e) => Err(ChipError::ReadFile(e.to_string()))
                }
            },
            Err(e) => Err(ChipError::ReadFile(e.to_string()))
        }
    }
}

impl Core for Emulator {
    fn run(&mut self) {
        let dur = time::Duration::from_micros(1_000_000 / self.clock);
        let mut win_size = self.api.window_size();

        while self.api.is_window_open() == true {
            // Handling events + get keyboard / mouse inputs
            let inputs = self.api.events();
        
            // The interpreter calls the current instruction
            let display = self.interpreter.step(inputs);
            let size_changed = self.api.window_size() != win_size;
            
            if display == true || size_changed == true {
                self.draw_vram();
                
                if size_changed {
                    win_size = self.api.window_size();
                }
                self.api.display();
            }

            self.try_beep();

            thread::sleep(dur);
        }
    }

    /// Draw the vram throught the graphical API
    fn draw_vram(&mut self) {
        let vram = self.interpreter.vram();
        let wsize = self.api.window_size();
        let (w, h) = (
            wsize.0 as usize / vram.w(),
            wsize.1 as usize / vram.h()
        );
        
        for (i, value) in vram.value().iter().enumerate() {
            // Rectangle size
            let x = ((i % vram.w()) * w) as i32;
            let y = ((i / vram.w()) * h) as i32;
            
            // Rectangle properties
            let rect = Rectangle::from((x, y, w as u32, h as u32));
            let color = if value & 1 == 1 {
                ColorPreset::White.into()
            } else {
                ColorPreset::Black.into()
            };

            // Draw the rectangle
            self.api.draw_rect(rect, color);
        }
    }

    fn try_beep(&mut self) {
        if self.interpreter.beep() == true {
            self.api.resume_beep();
        } else {
            self.api.pause_beep();
        }
    }
}
