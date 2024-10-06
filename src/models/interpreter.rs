use crate::{
    event::Input,
    models::{instructions::Instructions, memory::Memory},
    properties::vram::Vram,
};

/// Parse and interprets the raw byte instructions
pub trait Interpreter: Memory + Instructions {
    /// Returns the graphic screen memory
    fn vram(&self) -> Vram;
    /// It represents a tick (we can consider its a
    /// cycle assuming every instruction take only one cycle)
    ///
    /// If the return value is true then display the screen
    fn step(&mut self, inputs: Vec<Input>) -> bool;
    /// Load the program into the memory
    fn load_program(&mut self, program: Vec<u8>);
    /// Return if it has to beep
    fn beep(&self) -> bool;
    /// Force to use the original load semantic
    fn set_original_load(&mut self, value: bool);
    /// Force to use the original load semantic
    fn set_original_shift(&mut self, value: bool);
}
