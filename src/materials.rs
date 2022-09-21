use std::f32::consts::PI;

use crate::color::Color;
use crate::ray::Ray;
use crate::render::{Background, Intersection, Material};
use crate::scene::Scene;

const BIAS: f32 = 16. * f32::EPSILON;

pub struct DiffuseEmitter {
    pub color: Color,
}

impl Material for DiffuseEmitter {
    fn surface_color(&self, _scene: &Scene, _inter: Intersection, _bounce: u32) -> Color {
        self.color
    }
}

impl Background for DiffuseEmitter {
    fn background_color(&self, _: &Scene, _: Ray) -> Color {
        self.color
    }
}

pub struct SurfaceNormal {}

impl Material for SurfaceNormal {
    fn surface_color(&self, _scene: &Scene, inter: Intersection, _bounce: u32) -> Color {
        let surface_normal = inter.object.geometry.surface_normal(inter.hit_point());
        0.5 * Color {
            red: surface_normal.x,
            green: surface_normal.y,
            blue: surface_normal.z,
        }
    }
}

pub struct None {}

impl Material for None {
    fn surface_color(&self, _: &Scene, _: Intersection, _: u32) -> Color {
        Color::BLACK
    }
}

impl Background for None {
    fn background_color(&self, _: &Scene, _: Ray) -> Color {
        Color::BLACK
    }
}

trait Diffuse {
    fn albedo(&self) -> f32;
    fn color(&self) -> Color;

    fn diffuse_color(&self, scene: &Scene, inter: Intersection, bounce: u32) -> Color {
        let surface_normal = inter.object.geometry.surface_normal(inter.hit_point());

        if let Some((light_color, ray)) = scene.sample(inter, bounce + 1) {
            let light_power = (surface_normal * ray.direction).max(0.0);
            let light_reflected = self.albedo() / PI;
            let color = light_color * light_power * light_reflected;

            self.color() * color
        } else {
            Color::BLACK
        }
    }
}

trait Reflective {
    fn reflective_color(&self, scene: &Scene, inter: Intersection, bounce: u32) -> Color {
        let hit_point = inter.hit_point();
        let surface_normal = inter.object.geometry.surface_normal(hit_point);

        let reflection_ray = Ray {
            origin: hit_point + BIAS * surface_normal,
            direction: (inter.ray.direction
                - 2. * (inter.ray.direction * surface_normal) * surface_normal)
                .normalize(),
        };

        scene.color(reflection_ray, bounce + 1)
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
    fn surface_color(&self, scene: &Scene, inter: Intersection, bounce: u32) -> Color {
        if self.roughness == 1. {
            self.diffuse_color(scene, inter, bounce)
        } else if self.roughness == 0. {
            self.reflective_color(scene, inter, bounce)
        } else {
            self.roughness * self.diffuse_color(scene, inter, bounce)
                + (1. - self.roughness) * self.reflective_color(scene, inter, bounce)
        }
    }
}
