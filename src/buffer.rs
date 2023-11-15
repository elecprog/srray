use std::slice::IterMut;

use crate::color::Color;

// Basic sRGB and buffer implementation for headless projects

const SRGB_GAMMA: f64 = 2.2;
fn gamma_decode(encoded: f64) -> f64 {
    encoded.powf(SRGB_GAMMA)
}
fn gamma_encode(linear: f64) -> f64 {
    linear.powf(1.0 / SRGB_GAMMA)
}

impl From<[u8; 3]> for Color {
    fn from(rgb: [u8; 3]) -> Self {
        Color {
            red: gamma_decode((rgb[0] as f64) / 255.0),
            green: gamma_decode((rgb[1] as f64) / 255.0),
            blue: gamma_decode((rgb[2] as f64) / 255.0),
        }
    }
}

impl From<Color> for [u8; 3] {
    fn from(color: Color) -> Self {
        let col = color.clamp();
        [
            (gamma_encode(col.red) * 255.0) as u8,
            (gamma_encode(col.green) * 255.0) as u8,
            (gamma_encode(col.blue) * 255.0) as u8,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Buffer {
    cols: u32,
    rows: u32,
    // p_r,c = data[c * rows + row]
    data: Vec<[u8; 3]>,
}

impl Buffer {
    pub fn new(cols: u32, rows: u32) -> Self {
        Buffer {
            cols,
            rows,
            data: vec![[0, 0, 0]; (cols * rows) as usize],
        }
    }

    pub fn cols(&self) -> u32 {
        self.cols
    }
    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn cart_to_lin(rows: u32, row: u32, col: u32) -> u32 {
        col * rows + row
    }
    pub fn lin_to_cart(rows: u32, idx: u32) -> (u32, u32) {
        (idx % rows, idx / rows)
    }

    pub fn get(&self, row: u32, col: u32) -> [u8; 3] {
        self.data[Self::cart_to_lin(self.rows, row, col) as usize]
    }
    pub fn get_lin(&self, idx: u32) -> [u8; 3] {
        self.data[idx as usize]
    }
    pub fn get_mut<'a>(&'a mut self, row: u32, col: u32) -> &'a mut [u8; 3] {
        let idx = Self::cart_to_lin(self.rows, row, col);
        self.data.get_mut(idx as usize).unwrap()
    }
    pub fn get_lin_mut<'a>(&'a mut self, idx: u32) -> &'a mut [u8; 3] {
        self.data.get_mut(idx as usize).unwrap()
    }

    pub fn enum_iter_mut<'a>(&'a mut self) -> EnumBuffer<'a> {
        EnumBuffer {
            pixels: self.data.iter_mut(),
            rows: self.rows,
            idx: 0,
        }
    }
}

pub struct EnumBuffer<'a> {
    pixels: IterMut<'a, [u8; 3]>,
    rows: u32,
    idx: u32,
}

impl<'a> Iterator for EnumBuffer<'a> {
    type Item = (u32, u32, &'a mut [u8; 3]);

    fn next(&mut self) -> Option<Self::Item> {
        let (row, col) = Buffer::lin_to_cart(self.rows, self.idx);
        self.idx += 1;
        self.pixels.next().map(|p| (row, col, p))
    }
}
