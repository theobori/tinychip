/// Every possible instructions for CHIP8
pub trait Instructions {
    /// 00E0 - CLS
    ///
    /// Clear the screen
    fn cls(&mut self);
    /// 1nnn - JP addr
    /// 
    /// Jump to location nnn.
    /// The interpreter sets the program counter to nnn.
    fn jp(&mut self);
    /// 6xkk - LD Vx, byte
    /// 
    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    fn ld_vx(&mut self);
    /// 7xkk - ADD Vx, byte
    ///
    /// Set Vx = Vx + kk.
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn add_vx(&mut self);
    /// Annn - LD I, addr
    ///
    /// Set I = nnn.
    /// The value of register I is set to nnn.
    fn ld_i(&mut self);
    ///Dxyn - DRW Vx, Vy, nibble
    ///
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    /// The interpreter reads n bytes from memory, starting at the address stored in I.
    /// 
    /// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// 
    /// Sprites are XORed onto the existing screen.
    /// 
    /// If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    /// 
    /// If the sprite is positioned so part of it is outside the coordinates of the display,
    /// it wraps around to the opposite side of the screen.
    fn drw(&mut self);
}
