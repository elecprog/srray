use crate::color::Color;
use crate::ray::Ray;
use crate::render::{Background, Intersection, Object};

pub struct Scene<'a> {
    pub max_bounces: u32,
    pub objects: Vec<&'a Object<'a>>,
    pub background: &'a dyn Background,
}

impl<'a> Scene<'a> {
    // TODO: ray march
    pub fn trace(&self, ray: &'a Ray) -> Option<Intersection<'a>> {
        self.objects
            .iter()
            .filter_map(|object| {
                object.geometry.intersect(ray).map(|distance| Intersection {
                    ray,
                    object,
                    distance,
                })
            })
            .min_by(|inter1, inter2| inter1.distance.partial_cmp(&inter2.distance).unwrap())
    }

    pub fn color(&self, ray: &'a Ray, bounces: u32) -> Color {
        if bounces > self.max_bounces {
            self.background.background_color(self, ray)
        } else {
            match self.trace(ray) {
                Some(inter) => inter.object.material.surface_color(self, &inter, bounces),
                None => self.background.background_color(self, ray),
            }
        }
    }
}
