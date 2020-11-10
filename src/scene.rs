use crate::camera::Camera;
use crate::color::Color;
use crate::ray::Ray;
use crate::render::{Background, Intersection, Light, Object};

use image::{DynamicImage, GenericImage};

pub struct Scene<'a> {
    pub camera: Camera,
    pub max_bounces: u32,
    pub objects: Vec<&'a Object<'a>>,
    pub lights: Vec<&'a dyn Light>,
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

    pub fn render(&self) -> DynamicImage {
        let (width, height) = (self.camera.width, self.camera.height);
        let mut image = DynamicImage::new_rgba8(width, height);

        for x in 0..width {
            for y in 0..height {
                let ray = self.camera.create_prime(x, y);
                let color = self.color(&ray, 0);
                image.put_pixel(x, y, color.to_rgba());
            }
        }
        image
    }
}
