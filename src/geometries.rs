use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

pub trait Geometry: Sync {
    fn intersect(&self, ray: Ray) -> Option<f64>;
    fn surface_normal(&self, point: Point) -> Vector;
}

const BIAS: f64 = 4.0 * f64::EPSILON;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Geometry for Sphere {
    fn intersect(&self, ray: Ray) -> Option<f64> {
        // Allows for optimizations
        debug_assert!((ray.direction.norm() - 1.).abs() <= 4.0 * f64::EPSILON);

        let l = ray.origin - self.center;
        let b = ray.direction * l;
        let c = l.dot(l) - self.radius * self.radius;

        let d = b * b - c;

        if d > 0. {
            let t = b.abs() + d.sqrt();
            if b > 0. || c < 0. {
                // Negative distance
                None
            } else {
                Some(c / t)
            }
        } else if b <= 0. && d.abs() <= f64::EPSILON {
            Some(-b)
        } else {
            None
        }
    }

    fn surface_normal(&self, point: Point) -> Vector {
        (point - self.center).normalize()
    }
}

pub struct Plane {
    pub origin: Point,
    pub normal: Vector,
}

impl Geometry for Plane {
    fn intersect(&self, ray: Ray) -> Option<f64> {
        let denom = ray.direction * self.normal;
        if denom.abs() > BIAS {
            let oo = self.origin - ray.origin;
            let d = (oo * self.normal) / denom;
            if d >= 0. {
                return Some(d);
            }
        }
        None
    }

    fn surface_normal(&self, _point: Point) -> Vector {
        self.normal.normalize()
    }
}
