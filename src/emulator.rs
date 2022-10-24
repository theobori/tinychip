use std::{
    fs::File,
    path::Path,
    io::Read,
    thread,
    time
};

use crate::models::{
    core::Core,
    memory::Memory,
    graphic::Graphic
};

use crate::graphics::api::GraphicType;
use crate::error::ChipError;
use crate::properties::clock::{Clock, ClockState};

// impl Default for EmulatorBuilder {
//     fn default() -> Self {
//         let title = String::from("chip8");

//         Self {
//             graphic_api: GraphicType::Sdl(title, 800, 600)
//         }
//     }
// }

// /// Build a `Emulator` struct
// pub struct EmulatorBuilder {
//     graphic_api: GraphicType
// }

// impl EmulatorBuilder {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     /// Set the api type
//     pub fn set_api(mut self, api: GraphicType) -> Self {
//         self.graphic_api = api;

//         self
//     }

//     /// Build the emulator
//     pub fn build(self) -> Emulator {
//         Emulator {
//             graphic_api_type: self.graphic_api,
//             graphic_api: None
//         }
//     }
// }

/// Main structure that will contains almost everything
pub struct Emulator {
    /// Type of the graphic API
    pub graphic_api_type: GraphicType,
    graphic_api: Option<Box<dyn Graphic>>
}

impl Emulator {
    pub fn new(graphic_api_type: GraphicType) -> Self {
        Self {
            graphic_api_type,
            graphic_api: None
        }
    }

    /// Load program raw bytes
    pub fn load<T: Into<Vec<u8>>>(&mut self, program: T) -> &mut Self {
        // Call load from memory trait
        
        self
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
        let graphic_api = self.graphic_api.as_mut().unwrap();

        // Clocks
        let mut clock_event = Clock::new(50);
        // let mut clock_update = Clock::new(50);
        // let mut clock_display = Clock::new(50);

        while graphic_api.is_window_open() == true {
            // Handling events
            if clock_event.try_reset() == true {
                for input in graphic_api.events() {
                    println!("{:?}", input);
                }
            }

            // Call cycle from the interpreter
            
            // Display screen

            // Security program sleep
            thread::sleep(time::Duration::from_millis(10));
        }
    }

    fn init(&mut self) {
        let t = self.graphic_api_type.clone();

        self.graphic_api = Some(t.into());
    }
}
