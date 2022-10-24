pub trait Core {
    /// Initialize the core
    fn init(&mut self);
    /// Run the emulation
    fn run(&mut self);
}
