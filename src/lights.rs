use crate::color::Color;
use crate::point::Point;
use crate::render::Light;
use crate::vector::Vector;

pub struct DirectionalLight {
    pub direction: Vector,
    pub color: Color,
    pub intensity: f32,
}

impl Light for DirectionalLight {
    fn direction_to(&self, _point: &Point) -> Vector {
        self.direction.normalize()
    }

    fn intensity(&self, _point: &Point) -> f32 {
        self.intensity
    }

    fn color(&self, _point: &Point) -> Color {
        self.color
    }

    fn distance(&self, _point: &Point) -> f32 {
        std::f32::INFINITY
    }
}

pub struct PointLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}

impl Light for PointLight {
    fn direction_to(&self, point: &Point) -> Vector {
        (*point - self.position).normalize()
    }

    fn intensity(&self, point: &Point) -> f32 {
        self.intensity / (4. * std::f32::consts::PI * self.distance(point))
    }

    fn color(&self, _point: &Point) -> Color {
        self.color
    }

    fn distance(&self, point: &Point) -> f32 {
        (self.position - *point).length()
    }
}
