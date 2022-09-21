use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;
use crate::scene::Scene;

pub trait Light: Sync {
    fn color(&self, scene: &Scene, ray: Ray) -> Color;
    fn random_point(&self) -> (Point, f32);
}

pub struct PointLight {
    pub center: Point,
    pub color: Color,
}

impl Light for PointLight {
    fn color(&self, _: &Scene, _: Ray) -> Color {
        self.color
    }

    fn random_point(&self) -> (Point, f32) {
        (self.center, 1.0)
    }
}
