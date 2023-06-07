use crate::color::Color;
use crate::light::Light;
use crate::materials::Background;
use crate::objects::Object;
use crate::point::Point;
use crate::ray::Ray;

pub struct Scene {
    pub max_bounces: u32,
    pub objects: Vec<Object>,
    pub lights: Vec<Box<dyn Light>>,
    pub background: Box<dyn Background>,
}

impl Scene {
    pub fn trace(&self, ray: Ray) -> Option<(f64, &Object, Point)> {
        self.objects
            .iter()
            .filter_map(|object| {
                object
                    .geometry
                    .intersect(ray)
                    .map(|distance| (distance, object, ray.origin + distance * ray.direction))
            })
            .min_by(|inter1, inter2| inter1.0.partial_cmp(&inter2.0).unwrap())
    }

    pub fn see(&self, a: Point, b: Point) -> bool {
        let ray = Ray {
            origin: a,
            direction: (b - a).normalize(),
        };
        if let Some(inter) = self.trace(ray) {
            inter.0 > (b - a).norm()
        } else {
            true
        }
    }

    pub fn color(&self, ray: Ray, bounces: u32) -> Color {
        if bounces > self.max_bounces {
            self.background.background_color(self, ray)
        } else {
            match self.trace(ray) {
                Some(inter) => inter.1.material.surface_color(
                    self,
                    inter.2,
                    inter.1.geometry.surface_normal(inter.2),
                    ray,
                    bounces,
                ),
                None => self.background.background_color(self, ray),
            }
        }
    }
}
