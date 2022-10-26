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
    /// Considering COSMAC VIP that a 1 MHz 1802 CPU
    /// 
    /// Execute only one instruction
    fn execute(&mut self, inputs: Vec::<Input>);
    /// Load the program into the memory
    fn load_program(&mut self, program: Vec::<u8>);
}
