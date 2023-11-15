use std::{
    io::{Result, Write},
    slice::IterMut,
};

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
    width: u32,
    height: u32,
    // p_r,c = data[c * rows + row]
    data: Vec<[u8; 3]>,
}

impl Buffer {
    pub fn new(width: u32, height: u32) -> Self {
        Buffer {
            width,
            height,
            data: vec![[0, 0, 0]; (width * height) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cart_to_lin(height: u32, x: u32, y: u32) -> u32 {
        x * height + y
    }
    pub fn lin_to_cart(height: u32, idx: u32) -> (u32, u32) {
        (idx / height, idx % height)
    }

    pub fn get(&self, x: u32, y: u32) -> [u8; 3] {
        self.data[Self::cart_to_lin(self.height, x, y) as usize]
    }
    pub fn get_lin(&self, idx: u32) -> [u8; 3] {
        self.data[idx as usize]
    }
    pub fn get_mut<'a>(&'a mut self, x: u32, y: u32) -> &'a mut [u8; 3] {
        let idx = Self::cart_to_lin(self.height, x, y);
        self.data.get_mut(idx as usize).unwrap()
    }
    pub fn get_lin_mut<'a>(&'a mut self, idx: u32) -> &'a mut [u8; 3] {
        self.data.get_mut(idx as usize).unwrap()
    }

    pub fn enum_iter_mut<'a>(&'a mut self) -> EnumBufferMut<'a> {
        EnumBufferMut {
            pixels: self.data.iter_mut(),
            height: self.height,
            idx: 0,
        }
    }

    pub fn write_binary_ppm<W: Write>(&self, out: &mut W) -> Result<()> {
        // Write header
        write!(out, "P6\n")?;
        write!(out, "{} {} {}\n", self.width, self.height, u8::MAX)?;
        for y in 0..self.height {
            for x in 0..self.width {
                out.write_all(self.get(x, y).as_slice())?;
            }
        }
        out.flush()
    }

    pub fn write_ascii_ppm<W: Write>(&self, out: &mut W) -> Result<()> {
        // Write header
        write!(out, "P3\n")?;
        write!(out, "{} {} {}\n", self.width, self.height, u8::MAX)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let rgb = self.get(x, y);
                write!(out, "{} {} {}\n", rgb[0], rgb[1], rgb[2])?;
            }
        }
        out.flush()
    }
}

pub struct EnumBufferMut<'a> {
    pixels: IterMut<'a, [u8; 3]>,
    height: u32,
    idx: u32,
}

impl<'a> Iterator for EnumBufferMut<'a> {
    type Item = (u32, u32, &'a mut [u8; 3]);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = Buffer::lin_to_cart(self.height, self.idx);
        self.idx += 1;
        self.pixels.next().map(|p| (x, y, p))
    }
}
