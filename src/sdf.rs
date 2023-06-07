use crate::geometries::Geometry;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

pub trait SDF: Sync {
    fn distance(&self, point: Point) -> f64;

    fn gradient(&self, point: Point) -> Vector {
        // Estimate gradient using tetrahedron technique
        let pmm: Vector = Vector::I - Vector::J - Vector::K;
        let mmp: Vector = -Vector::I - Vector::J + Vector::K;
        let mpm: Vector = -Vector::I + Vector::J - Vector::K;
        let ppp: Vector = Vector::I + Vector::J + Vector::K;

        const EPS: f64 = 10e-8;

        self.distance(point + EPS * pmm) * pmm
            + self.distance(point + EPS * mmp) * mmp
            + self.distance(point + EPS * mpm) * mpm
            + self.distance(point + EPS * ppp) * ppp
    }
}

impl SDF for fn(Point) -> (f64, Vector) {
    fn distance(&self, point: Point) -> f64 {
        self(point).0
    }

    fn gradient(&self, point: Point) -> Vector {
        self(point).1
    }
}

impl SDF for fn(Point) -> f64 {
    fn distance(&self, point: Point) -> f64 {
        self(point)
    }
}

/// Primitives

pub struct SDFSphere {}

impl SDF for SDFSphere {
    fn distance(&self, point: Point) -> f64 {
        (point - Point::ORIGIN).norm() - 1.
    }

    fn gradient(&self, point: Point) -> Vector {
        (point - Point::ORIGIN).normalize()
    }
}

pub struct SDFBox {
    pub lengthx: f64,
    pub lengthy: f64,
    pub lengthz: f64,
}

impl SDF for SDFBox {
    fn distance(&self, point: Point) -> f64 {
        let q = point.into_vector().abs()
            - Vector {
                x: self.lengthx / 2.,
                y: self.lengthy / 2.,
                z: self.lengthz / 2.,
            };
        q.max(0.).norm().min(q.maxcomp())
    }
}

pub struct SDFCube {}

impl SDF for SDFCube {
    fn distance(&self, point: Point) -> f64 {
        let q = point.into_vector().abs() - 0.5;
        q.max(0.).norm().min(q.maxcomp())
    }
}

pub struct SDFCross {}

impl SDF for SDFCross {
    fn distance(&self, point: Point) -> f64 {
        let da = point.x.abs().max(point.y.abs());
        let db = point.y.abs().max(point.z.abs());
        let dc = point.z.abs().max(point.x.abs());
        2. * da.min(db).min(dc) - 1.
    }
}

// Operations

pub struct SDFUnion {
    pub sdfa: Box<dyn SDF>,
    pub sdfb: Box<dyn SDF>,
}

impl SDF for SDFUnion {
    fn distance(&self, point: Point) -> f64 {
        self.sdfa.distance(point).min(self.sdfb.distance(point))
    }
}

pub struct SDFIntersect {
    pub sdfa: Box<dyn SDF>,
    pub sdfb: Box<dyn SDF>,
}

impl SDF for SDFIntersect {
    fn distance(&self, point: Point) -> f64 {
        self.sdfa.distance(point).max(self.sdfb.distance(point))
    }
}

pub struct SDFSubtract {
    pub sdfa: Box<dyn SDF>,
    pub sdfb: Box<dyn SDF>,
}

impl SDF for SDFSubtract {
    fn distance(&self, point: Point) -> f64 {
        self.sdfa.distance(point).max(-self.sdfb.distance(point))
    }
}

trait SDFTransform {
    fn get_sdf(&self) -> &dyn SDF;
    fn transform(&self, scalar: f64) -> f64;
    fn inverse_transform(&self, point: Point) -> Point;
}

impl<T: Sync> SDF for T
where
    T: SDFTransform,
{
    fn distance(&self, point: Point) -> f64 {
        self.transform(self.get_sdf().distance(self.inverse_transform(point)))
    }
}
pub struct SDFTranslate {
    pub sdf: Box<dyn SDF>,
    pub shift: Vector,
}

impl SDFTransform for SDFTranslate {
    fn get_sdf(&self) -> &dyn SDF {
        self.sdf.as_ref()
    }

    fn transform(&self, scalar: f64) -> f64 {
        scalar
    }

    fn inverse_transform(&self, point: Point) -> Point {
        point - self.shift
    }
}

pub struct SDFRotate {
    pub sdf: Box<dyn SDF>,
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
    pub reference: Point,
}

impl SDFTransform for SDFRotate {
    fn get_sdf(&self) -> &dyn SDF {
        self.sdf.as_ref()
    }

    fn transform(&self, scalar: f64) -> f64 {
        scalar
    }

    fn inverse_transform(&self, point: Point) -> Point {
        self.reference
            + (point - self.reference)
                .rotate_about_x_axis(-self.pitch.to_radians())
                .rotate_about_y_axis(-self.yaw.to_radians())
                .rotate_about_z_axis(-self.roll.to_radians())
    }
}
pub struct SDFScale {
    pub sdf: Box<dyn SDF>,
    pub scale: f64,
}

impl SDFTransform for SDFScale {
    fn get_sdf(&self) -> &dyn SDF {
        self.sdf.as_ref()
    }

    fn transform(&self, scalar: f64) -> f64 {
        self.scale * scalar
    }

    fn inverse_transform(&self, point: Point) -> Point {
        Point {
            x: point.x / self.scale,
            y: point.y / self.scale,
            z: point.z / self.scale,
        }
    }
}

// Rendering

pub struct SDFMarcher {
    pub sdf: Box<dyn SDF>,
    pub max_iterations: u32,
    pub max_distance: f64,
    pub tolerance: f64,
}

impl Geometry for SDFMarcher
{
    fn intersect(&self, ray: Ray) -> Option<f64> {
        // Ray marching
        let mut t = 0.;
        for _ in 0..self.max_iterations {
            let d = self.sdf.distance(ray.origin + t * ray.direction);

            if d.abs() < self.tolerance * t {
                break;
            }

            t += 0.95 * d;

            if t > self.max_distance {
                t = 0.0;
                break;
            }
        }

        if t > 0.0
            && self
                .surface_normal(ray.origin + t * ray.direction)
                .dot(ray.direction)
                < 1e-8
        {
            Some(t)
        } else {
            None
        }
    }

    fn surface_normal(&self, point: Point) -> Vector {
        self.sdf.gradient(point).normalize()
    }
}
