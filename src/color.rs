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

    pub fn powf(self, n: f64) -> Color {
        Color {
            red: self.red.powf(n),
            green: self.green.powf(n),
            blue: self.blue.powf(n),
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
