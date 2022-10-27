use rand::{rngs::ThreadRng, Rng};


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
    },
    event::Hotkey
};

use crate::interpreters::pc::ProgramCount;

use super::pc::ProgramCountState;

///   font
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

/// Interpreter state
#[derive(PartialEq)]
pub enum InterpreterState {
    Running,
    WaitForKey
}

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
    pc: ProgramCount,
    /// Graphics
    screen: Screen,
    /// Delay timer
    delay_timer: u8,
    /// Sound timer
    sound_timer: u8,
    /// Stack
    stack: [u16; 16],
    /// Stack pointer
    sp: u16,
    /// Keys
    key: [u8; 16],
    /// Random Number Generator
    rng: ThreadRng,
    /// Interpreter state
    state: InterpreterState
}

impl Default for ChipInterpreter {
    fn default() -> Self {
        Self {
            memory: [0; 4096],
            opcode: 0x0000.into(),
            v: [0; 16],
            i: 0,
            pc: ProgramCount::from(0x200),
            screen: Screen::default(),
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            rng: rand::thread_rng(),
            state: InterpreterState::Running
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

    /// CPU register at index `x`
    fn vx(&self) -> u8 {
        self.v[self.opcode.x() as usize]
    }

    /// CPU register at index `y`
    fn vy(&self) -> u8 {
        self.v[self.opcode.y() as usize]
    }

    /// Set the CPU register at index `x`
    fn set_vx(&mut self, byte: u8) {
        self.v[self.opcode.x() as usize] = byte;
    }

    /// Set the CPU register at index `y`
    #[allow(dead_code)]
    fn set_vy(&mut self, byte: u8) {
        self.v[self.opcode.y() as usize] = byte;
    }

    /// Reset keys
    fn reset_keys(&mut self) {
        for i in 0..self.key.len() {
            self.key[i] = 0;
        }
    }

    /// Assign keyboard hotkeys to the CHIP8 hotkeys
    fn assign_keys(&mut self, keys: Vec::<usize>) {
        for index in keys {
            self.key[index] = 1;
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
    fn sys(&mut self) {
        self.jp();
    }

    fn cls(&mut self) {    
        self.screen.clear();
    }

    fn ret(&mut self) {
        let state = ProgramCountState::Jump(self.stack[0xf]);
        
        self.pc.set_state(state);
        // Handle underflow ?
        self.sp -= 1;
    }

    fn jp(&mut self) {
        let state = ProgramCountState::Jump(self.opcode.nnn());

        self.pc.set_state(state);
    }

    fn call(&mut self) {
        self.sp += 1;
        self.stack[0xf] = self.pc.value;
        
        let state = ProgramCountState::Jump(self.opcode.nnn());
        self.pc.set_state(state);
    }

    fn se_vx_byte(&mut self) {
        if self.vx() == self.opcode.kk() {
            self.pc.set_state(ProgramCountState::Skip);
        }
    }

    fn sne_vx_byte(&mut self) {
        if self.vx() != self.opcode.kk() {
            self.pc.set_state(ProgramCountState::Skip);
        }
    }

    fn se_vx_vy(&mut self) {
        if self.vx() == self.vy() {
            self.pc.set_state(ProgramCountState::Skip);
        }
    }

    fn ld_vx_byte(&mut self) {
        self.set_vx(self.opcode.kk());
    }

    fn add_vx_byte(&mut self) {
        let sum = self.vx() as u16 + self.opcode.kk() as u16;

        // Lowest bits to avoid overflow
        self.set_vx((sum & 0x00ff) as u8);
    }

    fn ld_vx_vy(&mut self) {
        self.set_vx(self.vy());
    }

    fn or_vx_vy(&mut self) {
        self.set_vx(self.vx() | self.vy());
    }

    fn and_vx_vy(&mut self) {
        self.set_vx(self.vx() & self.vy());
    }

    fn xor_vx_vy(&mut self) {
        self.set_vx(self.vx() ^ self.vy());
    }

    fn add_vx_vy(&mut self) {
        let sum = (self.vx() as u16) + (self.vy() as u16);

        self.v[0xf] = (sum > 0xff) as u8;

        // Lowest 8 bits
        self.set_vx((sum & 0x00ff) as u8);
    }

    fn sub_vx_vy(&mut self) {
        let vx = self.vx();
        let vy = self.vy();
        
        if vx > vy {
            self.v[0xf] = 1;
            self.set_vx(vx - vy);
        } else {
            self.v[0xf] = 0;
        }
    }

    fn shr_vx_vy(&mut self) {
        let vx = self.v[self.opcode.x() as usize];

        self.v[self.opcode.x() as usize] >>= 1;
        self.v[0xf] = (vx & 1 == 1) as u8;
    }

    fn shr_vx_vy_original(&mut self) {

    }

    fn subn_vx_vy(&mut self) {
        let vx = self.vx();
        let vy = self.vy();
        
        if vy > vx {
            self.v[0xf] = 1;
            self.set_vx(vy - vx);
        } else {
            self.v[0xf] = 0;
        }
    }

    fn shl_vx_vy(&mut self) {
        let vx = self.vx();
        let left_shift = (vx as u16) << 1;

        // Lowest 8 bits
        self.set_vx((left_shift & 0x00ff) as u8);
        self.v[0xf] = (vx & 1 == 1) as u8;
    }

    fn shl_vx_vy_original(&mut self) {
        
    }

    fn sne_vx_vy(&mut self) {
        if self.vx() != self.vy() {
            self.pc.set_state(ProgramCountState::Skip);
        }
    }

    fn ld_i(&mut self) {
        self.i = self.opcode.nnn();
    }

    fn jp_v(&mut self) {
        let addr = self.opcode.nnn() + (self.v[0] as u16);

        self.pc.set_state(ProgramCountState::Jump(addr));
    }

    fn rnd_vx_byte(&mut self) {
        let byte = self.rng.gen_range(0..0xff) & self.opcode.kk();

        self.set_vx(byte);
    }

    fn drw_vx_vy_n(&mut self) {
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

    fn skp_vx(&mut self) {
        if self.key[self.vx() as usize] == 1 {
            self.pc.set_state(ProgramCountState::Skip);
        }
    }

    fn sknp_vx(&mut self) {
        if self.key[self.vx() as usize] == 0 {
            self.pc.set_state(ProgramCountState::Skip);
        }
    }

    fn ld_vx_dt(&mut self) {
        self.set_vx(self.delay_timer);
    }

    fn ld_vx_k(&mut self) {
        self.state = InterpreterState::WaitForKey;
    }

    fn ld_dt_vx(&mut self) {
        self.delay_timer = self.vx();
    }

    fn ld_st_vx(&mut self) {
        self.sound_timer = self.vx();
    }

    fn add_i_vx(&mut self) {
        self.i += self.vx() as u16;
    }

    fn ld_f_vx(&mut self) {
        todo!()
    }

    fn ld_b_vx(&mut self) {
        todo!()
    }

    fn ld_i_vx(&mut self) {
        let bytes = self.v[0..(self.vx() as usize)].to_vec();

        self.write_any(bytes, self.i as usize);
    }

    fn ld_i_vx_original(&mut self) {
        todo!()
    }

    fn ld_vx_i(&mut self) {
        for i in 0..(self.vx() as usize) {
            let index = i + self.i as usize;

            self.v[i] = self.read_byte(index);
        }
    }

    fn ld_vx_i_original(&mut self) {
        todo!()
    }
}

impl Interpreter for ChipInterpreter {
    fn screen(&self) -> Screen {
        self.screen.clone()
    }

    fn step(&mut self, inputs: Vec::<Input>) {
        // Fetch the operation code
        self.opcode = self.read_short(self.pc.value as usize).into();

        let value = self.opcode.value;
        let keys = Input::to_keys(inputs);

        // Listening for ld_vx_k (fx0a)
        // aka (Hotkeys handling)
        if self.state == InterpreterState::WaitForKey {
            match keys.first() {
                Some(value) => {
                    self.set_vx(*value as u8);
                    self.state = InterpreterState::Running;
                },
                None => return,
            }
        }

        // Reset the program counter stat
        self.pc.reset_state();

        // Hotkeys handling
        // self.reset_keys();
        // self.assign_keys(keys);

        // Execute the operation code
        match (value & 0xf000) >> 12 {
            0x0 => {
                match value & 0x0fff {
                    0x00e0 => self.cls(),
                    _ => {}
                }
            },
            0x1 => self.jp(),
            0x6 => self.ld_vx_byte(),
            0x7 => self.add_vx_byte(),
            0xa => self.ld_i(),
            0xd => self.drw_vx_vy_n(),
            _ => {}
        };

        // update the program counter
        self.pc.step();
    }

    fn load_program(&mut self, program: Vec::<u8>) {
        self.write_any(program, 0x200);
    }
}
