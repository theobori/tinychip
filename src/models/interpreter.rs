use crate::{
    models::{
        memory::Memory,
        instructions::Instructions
    },
    event::Input,
    properties::screen::Screen
};

/// Parse and interprets the raw byte instructions
pub trait Interpreter: Memory + Instructions {
    /// Returns the graphic screen array
    fn screen(&self) -> Screen;
    /// Execute only one instruction
    fn step(&mut self, inputs: Vec::<Input>);
    /// Load the program into the memory
    fn load_program(&mut self, program: Vec::<u8>);
}
