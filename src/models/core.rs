/// Kernel (linking graphic, cpu and audio)
pub trait Core {
    /// Run the emulation
    fn run(&mut self);
}
