use image::RgbaImage;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::color::Color;
use crate::point::Point;
use crate::random::random_uniform;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vector;

pub struct Camera {
    pub origin: Point,
    pub azimuth: f32,
    pub altitude: f32,
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub min_samples: u32,
    pub max_samples: u32,
    pub relative_tolerance: f32,
}

impl Camera {
    fn create_random_prime(&self, px: u32, py: u32) -> Ray {
        debug_assert!(px < self.width);
        debug_assert!(py < self.height);

        let (dx, dy) = ((self.width as f32).recip(), (self.height as f32).recip());
        let (px, py) = ((px as f32) * dx - 0.5, 0.5 - (py as f32) * dy);
        let (x, y) = (px + random_uniform() * dx, py + random_uniform() * dy);

        let aspect_ratio = (self.width as f32) / (self.height as f32);

        let sensor_x = x * aspect_ratio;
        let sensor_y = y;
        let sensor_z = -self.fov.to_radians().tan().recip();

        // Rotations
        let (saz, caz) = self.azimuth.sin_cos();
        let x = caz * sensor_x - saz * sensor_z;
        let z = saz * sensor_x + caz * sensor_z;

        let (sal, cal) = self.altitude.sin_cos();
        let y = cal * sensor_y - sal * z;
        let z = sal * sensor_y + cal * z;

        Ray {
            origin: self.origin,
            direction: Vector { x, y, z }.normalize(),
        }
    }

    fn render_pixel(&self, scene: &Scene, x: u32, y: u32) -> Color {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        debug_assert!(self.max_samples > 0);

        let mut color = Color::BLACK;
        for sample in 1..=self.max_samples {
            let ray = self.create_random_prime(x, y);
            let new_color =
                (sample as f32).recip() * ((sample - 1) as f32 * color + scene.color(&ray, 0));

            if sample >= self.min_samples
                && (new_color - color).norm() < color.norm() * self.relative_tolerance
            {
                color = new_color;
                break;
            }
            color = new_color;
        }
        color
    }

    pub fn render(&self, scene: &Scene) -> RgbaImage {
        let mut image = RgbaImage::new(self.width, self.height);
        image
            .enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, pixel)| *pixel = self.render_pixel(scene, x, y).to_rgba());
        image
    }
}
