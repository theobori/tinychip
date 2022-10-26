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
    graphic::Graphic,
    interpreter::Interpreter
};
use crate::graphics::api::{
    GraphicProp,
    Api,
    RECTS_X,
    RECTS_Y,
    WINDOW_MIN_W,
    WINDOW_MIN_H
};
use crate::error::ChipError;
use crate::properties::{
    clock::Clock,
    color::ColorPreset,
    rectangle::Rectangle
};

impl Default for EmulatorBuilder {
    fn default() -> Self {
        Self {
            api_prop: GraphicProp {
                api: Api::Sdl,
                title: String::from("chip8"),
                size: (WINDOW_MIN_W, WINDOW_MIN_H)
            },
            interpreter: Box::new(ChipInterpreter::new())
        }
    }
}

/// Build a `Emulator` struct
pub struct EmulatorBuilder {
    api_prop: GraphicProp,
    interpreter: Box<dyn Interpreter>
}

impl EmulatorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the api type
    pub fn set_api(mut self, api_type: Api) -> Self {
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

    /// Build the emulator
    pub fn build(self) -> Emulator {
        Emulator {
            api: self.api_prop.into(),
            interpreter: self.interpreter
        }
    }
}

/// Main structure that will contains almost everything
pub struct Emulator {
    /// Chip8 interpreter
    interpreter: Box<dyn Interpreter>,
    /// Graphical API
    api: Box<dyn Graphic>
}

impl Emulator {
    /// Create a `Emulator` struct
    pub fn new(
        api_prop: GraphicProp,
        interpreter: Box<dyn Interpreter>
    ) -> Self {
        Self {
            // graphic_api_type,
            api: api_prop.into(),
            interpreter
        }
    }

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

    /// Display the screen throught the graphic API
    fn display_screen(&mut self) {
        let screen = self.interpreter.screen();
        let wsize = self.api.window_size();
        let (w, h) = (
            wsize.0 as usize / screen.w(),
            wsize.1 as usize / screen.h()
        );
        
        for (i, value) in screen.value().iter().enumerate() {
            // Rectangle size
            let x = ((i % screen.w()) * w) as i32;
            let y = ((i / screen.w()) * h) as i32;
            
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
}

impl Core for Emulator {
    fn run(&mut self) {
        // Clocks
        // TODO: clocks as struct properties
        let mut clock_event = Clock::new(50);
        let mut clock_display = Clock::new(50);
        let mut inputs = Vec::new();

        while self.api.is_window_open() == true {
            // Handling events + get keyboard/mouse inputs
            if clock_event.try_reset() == true {
                inputs = self.api.events();
        
                for input in &inputs {
                    println!("{:?}", input);
                }
            }

            // The interpreter calls the current instruction
            self.interpreter.step(inputs.clone());
            
            // Clear and display screen
            if clock_display.try_reset() == true {
                self.api.clear();
                self.display_screen();
                self.api.display();
            }
            
            // Security program sleep
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
