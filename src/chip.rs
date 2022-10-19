use std::{
    fs::File,
    path::Path,
    io::Read
};

use crate::models::{
    core::Core,
    memory::Memory,
    graphic::Graphic
};

use crate::error::ChipError;

/// Main structure that will contains almost everything
pub struct Chip {
    graphic: Box<dyn Graphic>
}

impl Chip {
    pub fn new<T: Into<Box<dyn Graphic>>>(graphic_api: T) -> Self {
        Self {
            graphic: graphic_api.into()
        }
    }

    /// Load program raw bytes
    pub fn load<T: Into<Vec<u8>>>(&mut self, program: T) -> &mut Self {
        // Call load from memory trait
        
        todo!()
    }

    /// Load a program from file
    pub fn load_from_file<P: AsRef<Path>>(
        &mut self, path: P
    ) -> Result<&mut Self, ChipError> {
        let f = File::open(path);
        
        match f {
            Ok(mut file) => {
                let mut data = Vec::<u8>::new();

                match file.read_to_end(data.as_mut()) {
                    Ok(_) => {
                        self.load(data);

                        Ok(self)
                    }
                    Err(e) => Err(ChipError::ReadFile(e.to_string()))
                }
            },
            Err(e) => Err(ChipError::ReadFile(e.to_string()))
        }
    }
}

impl Core for Chip {
    fn run(&mut self) {
        todo!()
    }
}

impl Memory for Chip {
    fn write_any(&mut self, bytes: Vec<u8>, index: u32) {
        todo!()
    }

    fn write_byte_at(&mut self, byte: u8, index: u32) {
        todo!()
    }

    fn read_byte(&self, index: u32) -> u8 {
        todo!()
    }
}
