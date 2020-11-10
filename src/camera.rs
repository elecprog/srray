use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Camera {
    pub origin: Point,
    pub azimuth: f32,
    pub altitude: f32,
    pub width: u32,
    pub height: u32,
    pub fov: f32,
}

impl Camera {
    pub fn create_prime(&self, x: u32, y: u32) -> Ray {
        assert!(x < self.width);
        assert!(y < self.height);
        assert!(self.width >= self.height);

        let fov_adjustment = self.fov.to_radians().tan();
        let aspect_ratio = (self.width as f32) / (self.height as f32);

        let sensor_x = ((x as f32 + 0.5) / self.width as f32 - 0.5) * aspect_ratio * fov_adjustment;
        let sensor_y = (0.5 - y as f32 / self.height as f32) * fov_adjustment;
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
}
