use image::{Pixel, Rgba};
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub const BLACK: Color = Color {
        red: 0.,
        green: 0.,
        blue: 0.,
    };

    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn clamp(self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
        }
    }

    pub const GAMMA: f64 = 2.2;

    fn gamma_encode(linear: f64) -> f64 {
        linear.powf(1.0 / Color::GAMMA)
    }

    fn gamma_decode(encoded: f64) -> f64 {
        encoded.powf(Color::GAMMA)
    }

    pub fn powf(self, n: f64) -> Color {
        Color {
            red: self.red.powf(n),
            green: self.green.powf(n),
            blue: self.blue.powf(n),
        }
    }

    pub fn to_rgba(self) -> Rgba<u8> {
        let col = self.clamp();
        Rgba([
            (Color::gamma_encode(col.red) * 255.0) as u8,
            (Color::gamma_encode(col.green) * 255.0) as u8,
            (Color::gamma_encode(col.blue) * 255.0) as u8,
            255,
        ])
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        let chans = rgba.channels();
        Color {
            red: Color::gamma_decode((chans[0] as f64) / 255.0),
            green: Color::gamma_decode((chans[1] as f64) / 255.0),
            blue: Color::gamma_decode((chans[2] as f64) / 255.0),
        }
    }

    pub fn norm(self) -> f64 {
        (self.red * self.red + self.green * self.green + self.blue * self.blue).sqrt()
    }

    pub fn norm_squared(self) -> f64 {
        self.red * self.red + self.green * self.green + self.blue * self.blue
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, other: f64) -> Color {
        Color {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}
