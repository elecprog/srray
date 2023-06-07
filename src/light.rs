use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;
use crate::scene::Scene;

pub trait Light: Sync {
    fn sample(&self, scene: &Scene, point: Point) -> (Color, Ray);
}

pub struct PointLight {
    pub center: Point,
    pub color: Color,
}

impl Light for PointLight {
    fn sample(&self, scene: &Scene, point: Point) -> (Color, Ray) {
        let to_obj = Ray {
            origin: self.center,
            direction: (point - self.center).normalize(),
        };

        if scene.see(point, self.center) {
            (self.color * (point - self.center).norm_squared().recip(), to_obj)
        } else {
            (Color::BLACK, to_obj)
        }
    }
}