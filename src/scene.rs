use crate::color::Color;
use crate::light::Light;
use crate::point::Point;
use crate::random::{random_index, random_uniform, sample_cos_hemisphere};
use crate::ray::Ray;
use crate::render::{Background, Intersection, Object};

const BIAS: f32 = 16. * f32::EPSILON;

pub struct Scene {
    pub max_bounces: u32,
    pub objects: Vec<Object>,
    pub lights: Vec<Box<dyn Light>>,
    pub background: Box<dyn Background>,
}

impl Scene {
    pub fn sample(&self, inter: Intersection, bounces: u32) -> Option<(Color, Ray)> {
        if random_uniform() < 0.5 {
            self.sample_lights(inter).map(|(c, r)| (2. * c, r))
        } else {
            let (color, ray) = self.sample_hemisphere(inter, bounces);
            Some((2. * color, ray))
        }
    }

    pub fn sample_lights(&self, inter: Intersection) -> Option<(Color, Ray)> {
        let hit_point = inter.hit_point();

        let n = self.lights.len();
        if n == 0 {
            return None;
        }

        let light = &self.lights[random_index(n)];
        let (light_point, point_prob) = light.random_point();

        let d = light_point - hit_point;
        let direction = d.normalize();
        let ray = Ray {
            origin: hit_point + direction * BIAS,
            direction: direction,
        };

        if self.see(ray.origin, light_point) {
            let light_color =
                light.color(self, ray) * (n as f32) * point_prob.recip() * d.dot(d).recip();
            Some((light_color, ray))
        } else {
            None
        }
    }

    pub fn sample_hemisphere(&self, inter: Intersection, bounces: u32) -> (Color, Ray) {
        let hit_point = inter.hit_point();
        let surface_normal = inter.object.geometry.surface_normal(hit_point);

        let (random_direction, ray_probability) = sample_cos_hemisphere(surface_normal);

        let random_ray = Ray {
            origin: hit_point + random_direction * BIAS,
            direction: random_direction,
        };
        let light_color = self.color(random_ray, bounces) * ray_probability.recip();

        (light_color, random_ray)
    }

    pub fn trace(&self, ray: Ray) -> Option<Intersection> {
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

    pub fn see(&self, a: Point, b: Point) -> bool {
        let ray = Ray {
            origin: a,
            direction: (b - a).normalize(),
        };
        if let Some(inter) = self.trace(ray) {
            inter.distance > (b - a).norm()
        } else {
            true
        }
    }

    pub fn color(&self, ray: Ray, bounces: u32) -> Color {
        if bounces > self.max_bounces {
            self.background.background_color(self, ray)
        } else {
            match self.trace(ray) {
                Some(inter) => inter.object.material.surface_color(self, inter, bounces),
                None => self.background.background_color(self, ray),
            }
        }
    }
}
