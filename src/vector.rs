use std::ops::{Add, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub const NULL: Vector = Vector {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    pub const I: Vector = Vector {
        x: 1.,
        y: 0.,
        z: 0.,
    };

    pub const J: Vector = Vector {
        x: 0.,
        y: 1.,
        z: 0.,
    };

    pub const K: Vector = Vector {
        x: 0.,
        y: 0.,
        z: 1.,
    };

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        debug_assert!(self.norm() != 0.0);
        self.norm().recip() * *self
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn kron(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn orthonormals(&self) -> (Vector, Vector) {
        let n = self.normalize();
        if (n.x - n.y).abs() > 0.25 && (n.x - n.z).abs() > 0.25 {
            let t = Vector {
                x: n.z - n.y,
                y: n.x - n.z,
                z: n.y - n.x,
            }
            .normalize();
            (t, n.cross(&t))
        } else {
            let t = Vector {
                x: n.z - n.y,
                y: n.x + n.z,
                z: -n.y - n.x,
            }
            .normalize();
            (t, n.cross(&t))
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector {
    type Output = f32;

    fn mul(self, other: Vector) -> f32 {
        self.dot(&other)
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        other * self
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
