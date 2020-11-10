use image::RgbaImage;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::color::Color;
use crate::point::Point;
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
    pub aa_factor: u8,
}

impl Camera {
    fn create_prime(&self, bx: u32, by: u32, sx: u8, sy: u8) -> Ray {
        let aa_factor = self.aa_factor as u32;
        let (x, y) = (bx * aa_factor + sx as u32, by * aa_factor + sy as u32);
        let (width, height) = (aa_factor * self.width, aa_factor * self.height);

        debug_assert!(x < width);
        debug_assert!(y < height);
        debug_assert!(width >= height);

        let fov_adjustment = self.fov.to_radians().tan();
        let aspect_ratio = (width as f32) / (height as f32);

        let sensor_x = ((x as f32 + 0.5) / width as f32 - 0.5) * aspect_ratio * fov_adjustment;
        let sensor_y = (0.5 - y as f32 / height as f32) * fov_adjustment;
        let sensor_z = -(0.5 as f32).sqrt();

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

        debug_assert!(self.aa_factor > 0);
        let aa_scale_factor = ((self.aa_factor * self.aa_factor) as f32).recip();

        let mut color = Color::BLACK;
        for sx in 0..self.aa_factor {
            for sy in 0..self.aa_factor {
                let ray = self.create_prime(x, y, sx, sy);
                color = color + aa_scale_factor * scene.color(&ray, 0);
            }
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
