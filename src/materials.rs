use crate::color::Color;
use crate::ray::Ray;
use crate::render::{Background, Intersection, Material};
use crate::scene::Scene;

const BIAS: f32 = 16. * f32::EPSILON;

pub struct SolidColor {
    pub color: Color,
}

impl Material for SolidColor {
    fn surface_color(&self, _scene: &Scene, _inter: &Intersection, _bounce: u32) -> Color {
        self.color
    }
}

impl Background for SolidColor {
    fn background_color(&self, _scene: &Scene, _ray: &Ray) -> Color {
        self.color
    }
}

trait Diffuse {
    fn albedo(&self) -> f32;
    fn color(&self) -> Color;

    fn diffuse_color(&self, scene: &Scene, inter: &Intersection) -> Color {
        let hit_point = inter.hit_point();
        let surface_normal = inter.object.geometry.surface_normal(&hit_point);

        let mut color = Color::BLACK;
        for light in &scene.lights {
            let direction_to_light = light.direction_from(&hit_point);

            let shadow_ray = Ray {
                origin: hit_point + direction_to_light * BIAS,
                direction: direction_to_light.normalize(),
            };

            let shadow_inter = scene.trace(&shadow_ray);
            if shadow_inter.is_none() || shadow_inter.unwrap().distance > light.distance(&hit_point)
            {
                let light_intensity = light.intensity(&hit_point);
                let light_power = (surface_normal * direction_to_light).max(0.0) * light_intensity;
                let light_reflected = self.albedo() / std::f32::consts::PI;

                let light_color = light.color(&hit_point) * light_power * light_reflected;
                color = color + self.color() * light_color;
            }
        }

        color.clamp()
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
        if self.roughness == 1. {
            self.diffuse_color(scene, inter)
        } else if self.roughness == 0. {
            self.reflective_color(scene, inter, bounce)
        } else {
            self.roughness * self.diffuse_color(scene, inter)
                + (1. - self.roughness) * self.reflective_color(scene, inter, bounce)
        }
    }
}
