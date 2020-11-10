use crate::color::Color;
use crate::ray::Ray;
use crate::render::{Background, Intersection, Material};
use crate::scene::Scene;

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

pub struct SimpleDiffuse {
    pub color: Color,
    pub albedo: f32,
}

impl Material for SimpleDiffuse {
    fn surface_color(&self, scene: &Scene, inter: &Intersection, _bounce: u32) -> Color {
        let mut color = Color::BLACK;
        let hit_point = inter.hit_point();
        let surface_normal = inter.object.geometry.surface_normal(&hit_point);

        for light in &scene.lights {
            let direction_to_light = light.direction_from(&hit_point);

            let shadow_ray = Ray {
                origin: hit_point + direction_to_light * 16. * f32::EPSILON,
                direction: direction_to_light.normalize(),
            };

            let shadow_inter = scene.trace(&shadow_ray);
            if shadow_inter.is_none() || shadow_inter.unwrap().distance > light.distance(&hit_point)
            {
                let light_intensity = light.intensity(&hit_point);
                let light_power = (surface_normal * direction_to_light).max(0.0) * light_intensity;
                let light_reflected = self.albedo / std::f32::consts::PI;

                let light_color = light.color(&hit_point) * light_power * light_reflected;
                color = color + self.color * light_color;
            }
        }

        color.clamp()
    }
}
