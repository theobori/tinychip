/// Audio controller
pub trait Audio {
    /// Resume the beep sound
    fn resume_beep(&mut self);
    /// Pause the beep sound
    fn pause_beep(&mut self);
}
