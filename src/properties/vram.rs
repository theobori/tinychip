use crate::apis::api::{
    RECTS_X,
    RECTS_Y
};

/// Manage the display memory as a matrice
#[derive(Clone)]
pub struct Vram {
    /// Array
    value: Vec::<u8>,
    /// Width
    w: usize,
    /// Height
    h: usize
}

impl Default for Vram {
    fn default() -> Self {
        Self::new(
            RECTS_X as usize,
            RECTS_Y as usize,
            0
        )
    }
}

impl Vram {
    pub fn new(w: usize, h: usize, byte: u8) -> Self {
        Self {
            value: vec![byte; w * h],
            w,
            h
        }
    }

    /// Clear the screen
    pub fn clear(&mut self) {
        for i in 0..(self.w * self.h) {
            self.value[i] = 0;
        }
    }

    /// Get the value
    pub fn value(&self) -> Vec::<u8> {
        self.value.clone()
    }

    /// Get the width
    pub fn w(&self) -> usize {
        self.w
    }

    /// Get the height
    pub fn h(&self) -> usize {
        self.h
    }

    /// Get the index
    pub fn index<T: Into<usize>>(&self, x: T, y: T) -> usize {
        (y.into() * self.w) + x.into()
    }

    /// Set a byte at the location (x, y)
    pub fn put<T: Into<usize>>(&mut self, x: T, y: T, byte: u8) {
        let index = self.index(x, y);

        self.value[index] = byte;
    }

    /// Return the byte at the location (x, y)
    pub fn get<T: Into<usize>>(&mut self, x: T, y: T) -> u8 {
        self.value[self.index(x, y)]
    }
}
