use image::RgbaImage;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::color::Color;
use crate::halton::Halton2Sequence;
use crate::point::Point;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vector;

pub struct Camera {
    pub origin: Point,
    pub azimuth: f64,
    pub altitude: f64,
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub spp: u32,
}

impl Camera {
    fn create_prime(&self, x: f64, y: f64) -> Ray {
        let aspect_ratio = (self.width as f64) / (self.height as f64);
        let sensor_direction = Vector {
            x: x * aspect_ratio,
            y,
            z: -self.fov.to_radians().tan().recip(),
        }
        .normalize();

        Ray {
            origin: self.origin,
            direction: sensor_direction
                .rotate_about_x_axis(self.azimuth.to_radians())
                .rotate_about_y_axis(self.altitude.to_radians()),
        }
    }

    fn render_pixel(&self, scene: &Scene, px: u32, py: u32) -> Color {
        debug_assert!(px < self.width);
        debug_assert!(py < self.height);

        let (dx, dy) = ((self.width as f64).recip(), (self.height as f64).recip());
        let (x, y) = ((px as f64) * dx - 0.5, 0.5 - (py as f64) * dy);

        let scale_factor = (self.spp as f64).recip();
        let mut color = Color::BLACK;
        for (prx, pry) in Halton2Sequence::new(self.spp, 2, 3) {
            color = color
                + scale_factor
                    * scene.color(self.create_prime(x + prx / 2. * dx, y + pry / 2. * dy), 0)
        }
        color
    }

    pub fn render_section(&self, scene: &Scene, xmin: u32, xmax: u32, ymin: u32, ymax: u32) -> RgbaImage {
        debug_assert!(xmax <= self.width);
        debug_assert!(xmin <= xmax);
        debug_assert!(ymax <= self.height);
        debug_assert!(ymin <= ymax);

        let mut image = RgbaImage::new(xmax - xmin, ymax - ymin);
        image
            .enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, pixel)| *pixel = self.render_pixel(scene, x + xmin, y + ymin).to_rgba());
        image
    }

    pub fn render(&self, scene: &Scene) -> RgbaImage {
        self.render_section(scene, 0, self.width, 0, self.height)
    }
}
