/// Kernel (linking graphic, cpu and audio)
pub trait Core {
    /// Draw the vram (from CPU) using the API
    fn draw_vram(&mut self);
    /// Try to play the beep sound using the API (CPU depending too)
    fn try_beep(&mut self);
    /// Run the emulation
    fn run(&mut self);
}
