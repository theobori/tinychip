use crate::{
    models::{
        interpreter::Interpreter,
        memory::Memory,
        instructions::Instructions
    },
    event::Input,
    properties::{
        opcode::Opcode,
        screen::Screen
    }
};

/// ChipInterpreter font
const FONT: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

/// First chip8 interpreter
pub struct ChipInterpreter {
    /// Memory
    /// 
    /// 0x000-0x1ff - Chip 8 interpreter
    /// 0x050-0x0a0 - Used for the built in 4x5 pixel font set (0-f)
    /// 0x200-0xfff - Program ROM and RAM
    memory: [u8; 4096],
    /// Current opcode
    opcode: Opcode,
    /// CPU Registers
    v: [u8; 16],
    /// Index register
    i: u16,
    /// Program count
    pc: u16,
    /// Program count next value
    pc_next: u16,
    /// Graphics
    screen: Screen,
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
            opcode: 0x0000.into(),
            v: [0; 16],
            i: 0,
            pc: 0x200,
            pc_next: 0,
            screen: Screen::default(),
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
        let mut interpreter = Self::default();

        // Load the font
        interpreter.write_any(FONT.to_vec(), 0);
        interpreter
    }

    /// Reset the pc next value
    fn pc_next_reset(&mut self) {
        self.pc_next = 0;
    }

    /// Set pc_next
    fn set_pc_next(&mut self, value: u16) {
        self.pc_next = value;
    }

    /// Update pc
    fn update_pc(&mut self, inc: u16) {
        if self.pc_next == 0 {
            self.pc += inc;
        } else {
            self.pc = self.pc_next;
        }
    }
}

impl Memory for ChipInterpreter {
    fn write_byte_at(&mut self, byte: u8, index: usize) {
        self.memory[index] = byte;
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
    fn cls(&mut self) {    
        self.screen.clear();
    }

    fn jp(&mut self) {
        self.set_pc_next(self.opcode.nnn());
    }

    fn ld_vx(&mut self) {
        self.v[self.opcode.x() as usize] = self.opcode.kk();
    }

    fn add_vx(&mut self) {
        let index = self.opcode.x() as usize;
        let sum = self.v[index] as u16 + self.opcode.kk() as u16;

        // Lowest bits to avoid overflow
        self.v[index] = (sum & 0x00ff) as u8;
    }

    fn ld_i(&mut self) {
        self.i = self.opcode.nnn();
    }

    fn drw(&mut self) {
        self.v[0xf] = 0;

        for i in 0..self.opcode.n() {
            let index = (self.i + i) as usize;
            let byte = self.memory[index];
            let y = self.v[self.opcode.y() as usize] + i as u8;
            
            for shift in 1..=7 {
                let x = self.v[self.opcode.x() as usize] + shift;
                let bit = byte >> (8 - shift) & 1;

                // Xor sprite on the screen
                if bit ^ self.screen.get(x, y) == 0 {
                    self.screen.put(x, y, 0);
                    self.v[0xf] = 1;
                } else {
                    self.screen.put(x, y, 1);
                }
            }
        }
    }
}

impl Interpreter for ChipInterpreter {
    fn screen(&self) -> Screen {
        self.screen.clone()
    }

    fn step(&mut self, inputs: Vec::<Input>) {
        // Fetch
        self.opcode = self.read_short(self.pc as usize).into();

        let value = self.opcode.value;

        // Reset the next program count
        self.pc_next_reset();

        // Execute
        match (value & 0xf000) >> 12 {
            0x0 => {
                match value & 0x0fff {
                    0x00e0 => self.cls(),
                    _ => {}
                }
            },
            0x1 => self.jp(),
            0x6 => self.ld_vx(),
            0x7 => self.add_vx(),
            0xa => self.ld_i(),
            0xd => self.drw(),
            _ => {}
        };

        self.update_pc(2);
    }

    fn load_program(&mut self, program: Vec::<u8>) {
        self.write_any(program, 0x200);
    }
}
