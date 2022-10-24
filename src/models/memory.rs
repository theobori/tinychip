pub trait Memory {
    /// Write a byte at index the `index`
    fn write_byte_at(&mut self, byte: u8, index: usize);
    /// Write n bytes at index the `index`
    fn write_any(&mut self, bytes: Vec<u8>, index: usize);
    /// Read a byte at the index `index`
    fn read_byte(&self, index: usize) -> u8;
    /// Read an unsigned short int
    fn read_short(&self, index: usize) -> u16;
}
