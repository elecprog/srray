use crate::point::Point;
use crate::ray::Ray;
use crate::render::Geometry;
use crate::vector::Vector;

const BIAS: f32 = 4. * f32::EPSILON;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        // Allows for optmisations
        debug_assert!((ray.direction.norm() - 1.).abs() <= BIAS);

        let l = ray.origin - self.center;
        let b = ray.direction * l;
        let c = l.norm() - self.radius * self.radius;

        let d = b * b - c;

        if d > 0. {
            let t = b.abs() + d.sqrt();
            if b > 0. || c < 0. {
                // Negative distance
                None
            } else {
                Some(c / t)
            }
        } else if b <= 0. && d.abs() <= f32::EPSILON {
            Some(-b)
        } else {
            None
        }
    }

    fn surface_normal(&self, point: &Point) -> Vector {
        (*point - self.center).normalize()
    }
}

pub struct Plane {
    pub origin: Point,
    pub normal: Vector,
}

impl Geometry for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
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

    fn surface_normal(&self, _point: &Point) -> Vector {
        self.normal.normalize()
    }
}
