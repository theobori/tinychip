use crate::{
    models::{
        interpreter::Interpreter,
        memory::Memory,
        instructions::Instructions
    }, event::Input
};

/// First chip8 interpreter
pub struct ChipInterpreter {
    /// Memory
    /// 
    /// 0x000-0x1ff - Chip 8 interpreter
    /// 0x050-0x0a0 - Used for the built in 4x5 pixel font set (0-f)
    /// 0x200-0xfff - Program ROM and RAM
    memory: [u8; 4096],
    /// Current opcode
    opcode: u16,
    /// CPU Registers
    v: [u8; 16],
    /// Index register
    i: u16,
    /// Program count
    pc: u16,
    /// Graphics
    screen: [u8; 64 * 32],
    /// Delay timer
    delay_timer: u8,
    /// Sound timer
    sound_timer: u8,
    /// Stack
    stack: [u16; 16],
    /// Stack pointer
    stack_point: u16,
    /// Keys
    key: [u8; 16]
}

impl Default for ChipInterpreter {
    fn default() -> Self {
        Self {
            memory: [0; 4096],
            opcode: 0x0000,
            v: [0; 16],
            i: 0,
            pc: 0x200,
            screen: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_point: 0,
            key: [0; 16]
        }
    }
}

impl ChipInterpreter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Memory for ChipInterpreter {
    fn write_byte_at(&mut self, byte: u8, index: usize) {
        self.memory[index] = byte;
    }

    fn write_any(&mut self, bytes: Vec<u8>, index: usize) {
        for i in 0..bytes.len() {
            self.write_byte_at(bytes[i], index + i);
        }
    }

    fn read_byte(&self, index: usize) -> u8 {
        self.memory[index]
    }

    fn read_short(&self, index: usize) -> u16 {
        let hi = self.read_byte(index) as u16;
        let lo = self.read_byte(index + 1) as u16;

        hi << 8 | lo
    }
}

impl Instructions for ChipInterpreter {

}

impl Interpreter for ChipInterpreter {
    fn screen(&mut self) -> [u8; 64 * 32] {
        self.screen
    }

    fn cycle(&mut self, inputs: Vec::<Input>) {
        // Load the font

        // Get the opcode at self.pc
        let opcode = self.read_byte(self.pc as usize);
    }

    fn load_program(&mut self, program: Vec::<u8>) {
        self.write_any(program, 0x200);
    }
}