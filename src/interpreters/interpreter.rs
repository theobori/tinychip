use rand::Rng;

use crate::{
    apis::api::{RECTS_X, RECTS_Y},
    event::Input,
    models::{instructions::Instructions, interpreter::Interpreter, memory::Memory},
    properties::{clock::Clock, opcode::Opcode, vram::Vram},
};

use crate::interpreters::pc::ProgramCount;

use super::pc::{ProgramCountState, OPCODE_SIZE};

/// Font
const FONT: [u8; 5 * 16] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
    0x90, 0x90, 0xf0, 0x10, 0x10, // 4
    0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
    0xf0, 0x10, 0x20, 0x40, 0x40, // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xf0, 0x80, 0x80, 0x80, 0xf0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
    0xf0, 0x80, 0xf0, 0x80, 0x80, // F
];

/// Interpreter state
#[derive(Debug, PartialEq)]
pub enum InterpreterState {
    Running,
    WaitForKey,
}

/// First chip8 interpreter
pub struct ChipInterpreter {
    /// Memory
    ///
    /// 0x000-0x1ff - Chip 8 interpreter
    /// 0x050-0x0a0 - Used for the built in 4x5 pixel font set (0-f)
    /// 0x200-0x0fff - Program ROM and RAM
    ram: [u8; 4096],
    /// Current opcode
    opcode: Opcode,
    /// CPU Registers
    v: [u8; 16],
    /// Index register
    i: u16,
    /// Program count
    pc: ProgramCount,
    /// Graphics
    vram: Vram,
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
    /// Interpreter state
    state: InterpreterState,
    /// Controlling the screen display
    display: bool,
    /// Load semantic
    original_load: bool,
    /// Shift semantic
    original_shift: bool,
    /// Timers clock
    timers_clock: Clock,
}

impl Default for ChipInterpreter {
    fn default() -> Self {
        Self {
            ram: [0; 4096],
            opcode: 0x0000.into(),
            v: [0; 16],
            i: 0,
            pc: ProgramCount::from(0x200),
            vram: Vram::default(),
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            state: InterpreterState::Running,
            display: false,
            original_load: false,
            original_shift: false,
            timers_clock: Clock::new(1_000_000 / 60),
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
    fn assign_keys(&mut self, keys: Vec<usize>) {
        for index in keys {
            self.key[index] = 1;
        }
    }

    fn timers_tick(&mut self) {
        if self.timers_clock.try_reset() == false {
            return;
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}

impl Memory for ChipInterpreter {
    fn write_byte_at(&mut self, byte: u8, index: usize) {
        self.ram[index] = byte;
    }

    fn read_byte(&self, index: usize) -> u8 {
        self.ram[index]
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
        self.vram.clear();

        self.display = true;
    }

    fn ret(&mut self) {
        self.sp -= 1;
        let state = ProgramCountState::Jump(self.stack[self.sp as usize]);

        self.pc.set_state(state);
    }

    fn jp(&mut self) {
        let state = ProgramCountState::Jump(self.opcode.nnn());

        self.pc.set_state(state);
    }

    fn call(&mut self) {
        self.stack[self.sp as usize] = self.pc.value + OPCODE_SIZE;

        let state = ProgramCountState::Jump(self.opcode.nnn());

        self.pc.set_state(state);
        self.sp += 1;
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
        self.set_vx((sum & 0xff) as u8);
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

        self.v[0x0f] = (sum > 0xff) as u8;

        // Lowest 8 bits
        self.set_vx((sum & 0xff) as u8);
    }

    fn sub_vx_vy(&mut self) {
        let vx = self.vx();
        let vy = self.vy();

        self.v[0x0f] = (vx > vy) as u8;
        self.set_vx(vx.wrapping_sub(vy));
    }

    fn shr_vx_vy(&mut self) {
        if self.original_shift == true {
            return self.shr_vx_vy_original();
        }

        let vx = self.v[self.opcode.x() as usize];

        self.v[0x0f] = vx & 1;
        self.set_vx(vx >> 1);
    }

    fn shr_vx_vy_original(&mut self) {
        let vx = self.v[self.opcode.x() as usize];

        self.v[0x0f] = vx & 1;
        self.set_vx(self.vy() >> 1);
    }

    fn subn_vx_vy(&mut self) {
        let vx = self.vx();
        let vy = self.vy();

        self.v[0x0f] = (vy > vx) as u8;
        self.set_vx(vy.wrapping_sub(vx));
    }

    fn shl_vx_vy(&mut self) {
        if self.original_shift == true {
            return self.shl_vx_vy_original();
        }

        let vx = self.vx();

        self.v[0x0f] = vx >> 7;
        self.set_vx(vx << 1);
    }

    fn shl_vx_vy_original(&mut self) {
        let vx = self.vx();

        self.v[0x0f] = vx >> 7;
        self.set_vx(self.vy() << 1);
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
        let byte = rand::thread_rng().gen::<u8>() & self.opcode.kk();

        self.set_vx(byte);
    }

    fn drw_vx_vy_n(&mut self) {
        self.v[0x0f] = 0;

        for byte in 0..(self.opcode.n() as usize) {
            let y = (self.vy() as usize + byte) % RECTS_Y as usize;
            let value = self.read_byte(self.i as usize + byte);

            for bit in 0..8 {
                let x = (self.vx() as usize + bit) % RECTS_X as usize;
                let color = value >> (7 - bit) & 1;
                let vram_byte = self.vram.get(x, y);

                self.v[0x0f] |= color & vram_byte;
                self.vram.put(x, y, vram_byte ^ color);
            }
        }

        self.display = true;
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
        self.i = (self.vx() as u16) * 5;
    }

    fn ld_b_vx(&mut self) {
        let vx = self.vx();
        let i = self.i as usize;

        self.write_byte_at(vx / 100, i);
        self.write_byte_at((vx % 100) / 10, i + 1);
        self.write_byte_at(vx % 10, i + 2);
    }

    fn ld_i_vx(&mut self) {
        if self.original_load == true {
            return self.ld_i_vx_original();
        }

        let bytes = self.v[0..=(self.opcode.x() as usize)].to_vec();

        self.write_any(bytes, self.i as usize);
    }

    fn ld_i_vx_original(&mut self) {
        for i in 0..=(self.opcode.x() as usize) {
            self.write_byte_at(self.v[i], self.i as usize);
            self.i += 1;
        }
    }

    fn ld_vx_i(&mut self) {
        if self.original_load == true {
            return self.ld_vx_i_original();
        }

        for i in 0..=(self.opcode.x() as usize) {
            let index = i + self.i as usize;

            self.v[i] = self.read_byte(index);
        }
    }

    fn ld_vx_i_original(&mut self) {
        for i in 0..=(self.opcode.x() as usize) {
            self.v[i] = self.read_byte(self.i as usize);
            self.i += 1;
        }
    }
}

impl Interpreter for ChipInterpreter {
    fn vram(&self) -> Vram {
        self.vram.clone()
    }

    fn step(&mut self, inputs: Vec<Input>) -> bool {
        let keys = Input::to_keys(inputs);

        // Reset the program counter and screen display
        self.pc.reset_state();
        self.display = false;

        // Hotkeys handling
        self.reset_keys();
        self.assign_keys(keys.clone());

        // Listening for ld_vx_k (fx0a)
        // aka (Hotkeys halting)
        if self.state == InterpreterState::WaitForKey {
            if let Some(value) = keys.first() {
                self.set_vx(*value as u8);
                self.state = InterpreterState::Running;
            }
            return self.display;
        }

        // Upodate both timers
        self.timers_tick();

        // Fetch the operation code
        self.opcode = self.read_short(self.pc.value as usize).into();

        // Execute the operation code
        match self.opcode.into() {
            (0x00, 0x00, 0x0e, 0x00) => self.cls(),
            (0x00, 0x00, 0x0e, 0x0e) => self.ret(),
            (0x00, _, _, _) => self.sys(),
            (0x01, _, _, _) => self.jp(),
            (0x02, _, _, _) => self.call(),
            (0x03, _, _, _) => self.se_vx_byte(),
            (0x04, _, _, _) => self.sne_vx_byte(),
            (0x05, _, _, 0x00) => self.se_vx_vy(),
            (0x06, _, _, _) => self.ld_vx_byte(),
            (0x07, _, _, _) => self.add_vx_byte(),
            (0x08, _, _, 0x00) => self.ld_vx_vy(),
            (0x08, _, _, 0x01) => self.or_vx_vy(),
            (0x08, _, _, 0x02) => self.and_vx_vy(),
            (0x08, _, _, 0x03) => self.xor_vx_vy(),
            (0x08, _, _, 0x04) => self.add_vx_vy(),
            (0x08, _, _, 0x05) => self.sub_vx_vy(),
            (0x08, _, _, 0x06) => self.shr_vx_vy(),
            (0x08, _, _, 0x07) => self.subn_vx_vy(),
            (0x08, _, _, 0x0e) => self.shl_vx_vy(),
            (0x09, _, _, 0x00) => self.sne_vx_vy(),
            (0x0a, _, _, _) => self.ld_i(),
            (0x0b, _, _, _) => self.jp_v(),
            (0x0c, _, _, _) => self.rnd_vx_byte(),
            (0x0d, _, _, _) => self.drw_vx_vy_n(),
            (0x0e, _, 0x09, 0x0e) => self.skp_vx(),
            (0x0e, _, 0x0a, 0x01) => self.sknp_vx(),
            (0x0f, _, 0x00, 0x07) => self.ld_vx_dt(),
            (0x0f, _, 0x01, 0x05) => self.ld_dt_vx(),
            (0x0f, _, 0x01, 0x08) => self.ld_st_vx(),
            (0x0f, _, 0x01, 0x0e) => self.add_i_vx(),
            (0x0f, _, 0x02, 0x09) => self.ld_f_vx(),
            (0x0f, _, 0x03, 0x03) => self.ld_b_vx(),
            (0x0f, _, 0x00, 0x0a) => self.ld_vx_k(),
            (0x0f, _, 0x05, 0x05) => self.ld_i_vx(),
            (0x0f, _, 0x06, 0x05) => self.ld_vx_i(),
            (_, _, _, _) => {}
        }

        // Update the program counter
        self.pc.step();

        self.display
    }

    fn load_program(&mut self, program: Vec<u8>) {
        self.write_any(program, 0x200);
    }

    fn beep(&self) -> bool {
        self.sound_timer > 0
    }

    fn set_original_load(&mut self, value: bool) {
        self.original_load = value;
    }

    fn set_original_shift(&mut self, value: bool) {
        self.original_shift = value;
    }
}
