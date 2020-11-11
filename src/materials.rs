use std::f32::consts::PI;

use crate::random::random_uniform;

use crate::color::Color;
use crate::ray::Ray;
use crate::render::{Background, Intersection, Material};
use crate::scene::Scene;
use crate::vector::Vector;

const BIAS: f32 = 16. * f32::EPSILON;

pub struct DiffuseEmitter {
    pub color: Color,
}

impl Material for DiffuseEmitter {
    fn surface_color(&self, _scene: &Scene, _inter: &Intersection, _bounce: u32) -> Color {
        self.color
    }
}

impl Background for DiffuseEmitter {
    fn background_color(&self, _scene: &Scene, _ray: &Ray) -> Color {
        self.color
    }
}

pub struct SurfaceNormal {}

impl Material for SurfaceNormal {
    fn surface_color(&self, _scene: &Scene, inter: &Intersection, _bounce: u32) -> Color {
        let surface_normal = inter.object.geometry.surface_normal(&inter.hit_point());
        0.5 * Color {
            red: surface_normal.x,
            green: surface_normal.y,
            blue: surface_normal.z,
        }
    }
}

trait Diffuse {
    fn albedo(&self) -> f32;
    fn color(&self) -> Color;

    fn diffuse_color(&self, scene: &Scene, inter: &Intersection, bounce: u32) -> Color {
        let hit_point = inter.hit_point();
        let surface_normal = inter.object.geometry.surface_normal(&hit_point);

        let (random_direction, ray_probability) = Vector::sample_cos_hemisphere(surface_normal);

        let random_ray = Ray {
            origin: hit_point + random_direction * BIAS,
            direction: random_direction,
        };

        let light_power = (surface_normal * random_direction).max(0.0);
        let light_reflected = self.albedo() / PI;
        let light_color = scene.color(&random_ray, bounce + 1)
            * light_power
            * light_reflected
            * ray_probability.recip();
        self.color() * light_color
    }
}

trait Reflective {
    fn reflective_color(&self, scene: &Scene, inter: &Intersection, bounce: u32) -> Color {
        let hit_point = inter.hit_point();
        let surface_normal = inter.object.geometry.surface_normal(&hit_point);

        let reflection_ray = Ray {
            origin: hit_point + BIAS * surface_normal,
            direction: (inter.ray.direction
                - 2. * (inter.ray.direction * surface_normal) * surface_normal)
                .normalize(),
        };

        scene.color(&reflection_ray, bounce + 1)
    }
}

pub struct SimpleMaterial {
    pub color: Color,
    pub albedo: f32,
    pub roughness: f32,
}

impl Diffuse for SimpleMaterial {
    fn albedo(&self) -> f32 {
        self.albedo
    }

    fn color(&self) -> Color {
        self.color
    }
}

impl Reflective for SimpleMaterial {}

impl Material for SimpleMaterial {
    fn surface_color(&self, scene: &Scene, inter: &Intersection, bounce: u32) -> Color {
        if self.roughness >= random_uniform() {
            self.diffuse_color(scene, inter, bounce)
        } else {
            self.reflective_color(scene, inter, bounce)
        }
    }
}
