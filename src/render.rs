use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vector;

pub struct Object<'a> {
    pub geometry: &'a dyn Geometry,
    pub material: &'a dyn Material,
}

pub trait Geometry: Sync {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn surface_normal(&self, point: &Point) -> Vector;
}

pub struct Intersection<'a> {
    pub ray: &'a Ray,
    pub object: &'a Object<'a>,
    pub distance: f32,
}

impl Intersection<'_> {
    pub fn hit_point(&self) -> Point {
        self.ray.origin + (self.distance * self.ray.direction)
    }
}

pub trait Material: Sync {
    fn surface_color(&self, scene: &Scene, inter: &Intersection, depth: u32) -> Color;
}

pub trait Light: Sync {
    fn direction_to(&self, point: &Point) -> Vector;
    fn intensity(&self, point: &Point) -> f32;
    fn color(&self, point: &Point) -> Color;
    fn distance(&self, point: &Point) -> f32;

    fn direction_from(&self, point: &Point) -> Vector {
        -self.direction_to(point)
    }
}

pub trait Background: Sync {
    fn background_color(&self, scene: &Scene, ray: &Ray) -> Color;
}
