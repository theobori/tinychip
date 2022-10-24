use crate::{
    models::{
        memory::Memory,
        instructions::Instructions
    },
    event::Input
};

/// Parse and interprets the raw byte instructions
pub trait Interpreter: Memory + Instructions {
    /// Returns the graphic screen array
    fn screen(&mut self) -> [u8; 64 * 32];
    /// Interprets the current instruction and move the program counter to the next one
    fn cycle(&mut self, inputs: Vec::<Input>);
    /// Load the program into the memory
    fn load_program(&mut self, program: Vec::<u8>);
}
