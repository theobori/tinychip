/// Contains some methods to extract arguments, etc..
pub struct Opcode {
    /// The current 2 bytes code
    pub value: u16
}

impl Opcode {
    pub fn new(value: u16) -> Self {
        Self {
            value
        }
    }

    // Conventional arguments

    /// Return the address
    pub fn nnn(&self) -> u16 {
        self.value & 0x0fff
    }

    /// Return the nibble
    pub fn n(&self) -> u16 {
        self.value & 0x000f
    }

    /// Return x
    pub fn x(&self) -> u16 {
        (self.value & 0x0f00) >> 8
    }

    /// Return y
    pub fn y(&self) -> u16 {
        (self.value & 0x00f0) >> 4
    }

    /// Return kk
    pub fn kk(&self) -> u8 {
        (self.value & 0x00ff) as u8
    }
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        Self {
            value
        }
    }
}
