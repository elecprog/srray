use image::{Pixel, Rgba};
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub const BLACK: Color = Color {
        red: 0.,
        green: 0.,
        blue: 0.,
    };

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }

    const GAMMA: f32 = 2.2;

    fn gamma_encode(linear: f32) -> f32 {
        linear.powf(1.0 / Color::GAMMA)
    }

    fn gamma_decode(encoded: f32) -> f32 {
        encoded.powf(Color::GAMMA)
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        let col = self.clamp();
        Rgba::from_channels(
            (Color::gamma_encode(col.red) * 255.0) as u8,
            (Color::gamma_encode(col.green) * 255.0) as u8,
            (Color::gamma_encode(col.blue) * 255.0) as u8,
            255,
        )
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        let chans = rgba.channels();
        Color {
            red: Color::gamma_decode((chans[0] as f32) / 255.0),
            green: Color::gamma_decode((chans[1] as f32) / 255.0),
            blue: Color::gamma_decode((chans[2] as f32) / 255.0),
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.green,
            green: self.green * other.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}
