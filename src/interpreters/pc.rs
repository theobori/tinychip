/// Program count actions
pub enum ProgramCountState {
    Next,
    Jump(u16),
    Skip,
}

impl Default for ProgramCountState {
    fn default() -> Self {
        ProgramCountState::Next
    }
}

/// Size of an operation code
pub const OPCODE_SIZE: u16 = 2;

/// Program Count
pub struct ProgramCount {
    /// Count
    pub value: u16,
    /// State
    pub state: ProgramCountState,
}

impl ProgramCount {
    /// Reset the state
    pub fn reset_state(&mut self) {
        self.state = ProgramCountState::default();
    }

    /// Set the state
    pub fn set_state(&mut self, state: ProgramCountState) {
        self.state = state;
    }

    /// Update
    pub fn step(&mut self) {
        match self.state {
            ProgramCountState::Next => self.value += OPCODE_SIZE,
            ProgramCountState::Jump(addr) => self.value = addr,
            ProgramCountState::Skip => self.value += OPCODE_SIZE * 2,
        }
    }
}

impl From<u16> for ProgramCount {
    fn from(value: u16) -> Self {
        Self {
            value,
            state: ProgramCountState::default(),
        }
    }
}
