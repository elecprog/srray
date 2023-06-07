use std::ops::{Add, Mul, Neg, Sub, Rem};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn norm_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Vector {
        let norm = self.norm();
        if norm != 0.0 {
            norm.recip() * self
        } else {
            self
        }
    }

    pub fn dot(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn kron(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn cross(self, other: Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn orthonormals(self) -> (Vector, Vector) {
        let n = self.normalize();
        if (n.x - n.y).abs() > 0.25 && (n.x - n.z).abs() > 0.25 {
            let t = Vector {
                x: n.z - n.y,
                y: n.x - n.z,
                z: n.y - n.x,
            }
            .normalize();
            (t, n.cross(t))
        } else {
            let t = Vector {
                x: n.z - n.y,
                y: n.x + n.z,
                z: -n.y - n.x,
            }
            .normalize();
            (t, n.cross(t))
        }
    }

    pub fn map(self, f: fn(f64) -> f64) -> Vector {
        Vector {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    pub fn abs(self) -> Vector {
        Vector {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn max(self, other: f64) -> Vector {
        Vector {
            x: self.x.max(other),
            y: self.y.max(other),
            z: self.z.max(other),
        }
    }

    pub fn maxcomp(self) -> f64 {
        self.x.max(self.y).max(self.z)
    }

    pub fn rem_euclid(self, other: f64) -> Vector {
        Vector {
            x: self.x.rem_euclid(other),
            y: self.y.rem_euclid(other),
            z: self.z.rem_euclid(other),
        }
    }

    pub fn rotate_about_x_axis(self, angle: f64) -> Vector {
        let (s, c) = angle.sin_cos();
        Vector {
            x: self.x,
            y: c * self.y - s * self.z,
            z: s * self.y + c * self.z,
        }
    }

    pub fn rotate_about_y_axis(self, angle: f64) -> Vector {
        let (s, c) = angle.sin_cos();
        Vector {
            x: c * self.x + s * self.z,
            y: self.y,
            z: -s * self.x + c * self.z,
        }
    }

    pub fn rotate_about_z_axis(self, angle: f64) -> Vector {
        let (s, c) = angle.sin_cos();
        Vector {
            x: c * self.x - s * self.y,
            y: s * self.x + c * self.y,
            z: self.z,
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

impl Add<Vector> for f64 {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z,
        }
    }
}

impl Add<f64> for Vector {
    type Output = Vector;

    fn add(self, other: f64) -> Vector {
        other + self
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

impl Sub<Vector> for f64 {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self - other.x,
            y: self - other.y,
            z: self - other.z,
        }
    }
}

impl Sub<f64> for Vector {
    type Output = Vector;

    fn sub(self, other: f64) -> Vector {
        Vector {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Mul for Vector {
    type Output = f64;

    fn mul(self, other: Vector) -> f64 {
        self.dot(other)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        other * self
    }
}

impl Rem<f64> for Vector {
    type Output = Vector;

    fn rem(self, other: f64) -> Vector {
        Vector {
            x: self.x % other,
            y: self.y % other,
            z: self.z % other,
        }
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
