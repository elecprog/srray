use crate::point::Point;
use crate::ray::Ray;
use crate::render::Geometry;
use crate::vector::Vector;

const EPS_GRAD: f32 = 1e-5;
const MAX_ITERS: i16 = 128;
const EPS_SDF: f32 = 1e-4;
const MAX_DIST: f32 = 1e12;

pub trait SDF: Sync {
    fn distance(&self, point: &Point) -> f32;
}

impl<S> Geometry for S
where
    S: SDF,
{
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        // Ray marching
        let mut t = 0.0;
        for _ in 0..MAX_ITERS {
            let p = ray.origin + t * ray.direction;
            let d = self.distance(&p);

            if d.abs() < EPS_SDF * t {
                break;
            }

            t += 0.75 * d;

            if t > MAX_DIST {
                t = 0.0;
                break;
            }
        }

        if t > 0.0
            && self
                .surface_normal(&(ray.origin + t * ray.direction))
                .dot(&ray.direction)
                .abs()
                > EPS_GRAD
        {
            Some(t)
        } else {
            None
        }
    }

    fn surface_normal(&self, point: &Point) -> Vector {
        // Estimate gradient
        let ip = self.distance(&(*point + EPS_GRAD * Vector::I));
        let im = self.distance(&(*point - EPS_GRAD * Vector::I));
        let jp = self.distance(&(*point + EPS_GRAD * Vector::J));
        let jm = self.distance(&(*point - EPS_GRAD * Vector::J));
        let kp = self.distance(&(*point + EPS_GRAD * Vector::K));
        let km = self.distance(&(*point - EPS_GRAD * Vector::K));
        Vector {
            x: ip - im,
            y: jp - jm,
            z: kp - km,
        }
        .normalize()
    }
}

pub struct SDFSphere {
    pub center: Point,
    pub radius: f32,
}

impl SDF for SDFSphere {
    fn distance(&self, point: &Point) -> f32 {
        (*point - self.center).norm() - self.radius
    }
}
