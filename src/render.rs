use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vector;

pub struct Object {
    pub geometry: Box<dyn Geometry>,
    pub material: Box<dyn Material>,
}

pub trait Geometry: Sync {
    fn intersect(&self, ray: Ray) -> Option<f32>;
    fn surface_normal(&self, point: Point) -> Vector;
}

#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    pub ray: Ray,
    pub object: &'a Object,
    pub distance: f32,
}

impl Intersection<'_> {
    pub fn hit_point(self) -> Point {
        self.ray.origin + (self.distance * self.ray.direction)
    }
}

pub trait Material: Sync {
    fn surface_color(&self, scene: &Scene, inter: Intersection, depth: u32) -> Color;
}

pub trait Background: Sync {
    fn background_color(&self, scene: &Scene, ray: Ray) -> Color;
}
