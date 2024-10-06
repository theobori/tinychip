use std::fmt::Debug;

/// Contains some methods to extract arguments, etc..
#[derive(Clone, Copy)]
pub struct Opcode {
    /// The current 2 bytes code
    pub value: u16,
}

impl Opcode {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    // Conventional arguments

    /// Return the address
    pub fn nnn(&self) -> u16 {
        self.value & 0x0fff
    }

    /// Return the nibble
    pub fn n(&self) -> u8 {
        (self.value & 0x000f) as u8
    }

    /// Return x
    pub fn x(&self) -> u8 {
        ((self.value & 0x0f00) >> 8) as u8
    }

    /// Return y
    pub fn y(&self) -> u8 {
        ((self.value & 0x00f0) >> 4) as u8
    }

    /// Return kk
    pub fn kk(&self) -> u8 {
        (self.value & 0x00ff) as u8
    }
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<Opcode> for (u8, u8, u8, u8) {
    fn from(opcode: Opcode) -> Self {
        (
            (opcode.value >> 12) as u8,
            opcode.x(),
            opcode.y(),
            opcode.n(),
        )
    }
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (first, x, y, n) = (*self).into();
        let ret = format!("{:#04x} {:#04x} {:#04x} {:#04x}", first, x, y, n);

        f.write_str(&ret)?;

        Ok(())
    }
}
