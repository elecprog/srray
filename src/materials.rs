use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vector;

pub trait Material: Sync {
    fn surface_color(
        &self,
        scene: &Scene,
        point: Point,
        normal: Vector,
        ray: Ray,
        bounces: u32,
    ) -> Color;
}

pub trait Background: Sync {
    fn background_color(&self, scene: &Scene, ray: Ray) -> Color;
}

const BIAS: f64 = 32. * f64::EPSILON;

pub struct DiffuseEmitter {
    pub color: Color,
}

impl Material for DiffuseEmitter {
    fn surface_color(
        &self,
        _scene: &Scene,
        _point: Point,
        _normal: Vector,
        _ray: Ray,
        _bounces: u32,
    ) -> Color {
        self.color
    }
}

impl Background for DiffuseEmitter {
    fn background_color(&self, _scene: &Scene, _ray: Ray) -> Color {
        self.color
    }
}

pub struct SurfaceNormal {}

impl Material for SurfaceNormal {
    fn surface_color(
        &self,
        _scene: &Scene,
        _point: Point,
        normal: Vector,
        _ray: Ray,
        _bounces: u32,
    ) -> Color {
        Color::new((normal.x + 1.) / 2., (normal.y + 1.) / 2., normal.z).powf(Color::GAMMA)
    }
}

pub struct DepthMap {}

impl Material for DepthMap {
    fn surface_color(
        &self,
        _scene: &Scene,
        point: Point,
        _normal: Vector,
        _ray: Ray,
        _bounces: u32,
    ) -> Color {
        let gray = point.z.rem_euclid(1.) / 1.5 + 0.1;
        Color::new(gray, gray, gray).powf(Color::GAMMA)
    }
}

pub struct None {}

impl Material for None {
    fn surface_color(
        &self,
        _scene: &Scene,
        _point: Point,
        _normal: Vector,
        _ray: Ray,
        _bounces: u32,
    ) -> Color {
        Color::BLACK
    }
}

impl Background for None {
    fn background_color(&self, _scene: &Scene, _ray: Ray) -> Color {
        Color::BLACK
    }
}

pub struct PhongMaterial {
    pub ambient_color: Color,
    pub diffuse_color: Color,
    pub specular_color: Color,
    pub specular_power: f64,
}

impl Material for PhongMaterial {
    fn surface_color(
        &self,
        scene: &Scene,
        point: Point,
        normal: Vector,
        ray: Ray,
        _bounces: u32,
    ) -> Color {
        let mut color = self.ambient_color;

        for light in &scene.lights {
            let (light_color, light_ray) = light.sample(scene, point + normal * BIAS);

            // Diffuse
            let light_power = (normal * -light_ray.direction).max(0.0);
            let light_color = light_color * light_power;
            color = color + self.diffuse_color * light_color;

            // Specular
            let h = (-light_ray.direction - ray.direction).normalize();
            let specular_light_power = (h * normal).max(0.).powf(self.specular_power);
            let specular_light_color = light_color * specular_light_power;
            color = color + self.specular_color * specular_light_color;
        }
        color
    }
}

pub struct Mirror {}

impl Material for Mirror {
    fn surface_color(
        &self,
        scene: &Scene,
        point: Point,
        normal: Vector,
        ray: Ray,
        bounces: u32,
    ) -> Color {
        let reflection_ray = Ray {
            origin: point + BIAS * normal,
            direction: (ray.direction - 2. * (ray.direction * normal) * normal).normalize(),
        };

        scene.color(reflection_ray, bounces + 1)
    }
}

pub struct MixedMaterial<'a> {
    material1: &'a dyn Material,
    material2: &'a dyn Material,
    mix: f64,
}

impl Material for MixedMaterial<'_> {
    fn surface_color(
        &self,
        scene: &Scene,
        point: Point,
        normal: Vector,
        ray: Ray,
        bounces: u32,
    ) -> Color {
        let color1 = self
            .material1
            .surface_color(scene, point, normal, ray, bounces);
        let color2 = self
            .material2
            .surface_color(scene, point, normal, ray, bounces);
        (1. - self.mix) * color1 + self.mix * color2
    }
}
