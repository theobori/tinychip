pub trait Memory {
    /// Write a byte at index the `index`
    fn write_byte_at(&mut self, byte: u8, index: u32);
    /// Write n bytes at index the `index`
    fn write_any(&mut self, bytes: Vec<u8>, index: u32);
    /// Read a byte at the index `index`
    fn read_byte(&self, index: u32) -> u8;
}
