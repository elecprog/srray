mod camera;
mod color;
mod geometries;
mod light;
mod materials;
mod point;
mod random;
mod ray;
mod render;
mod scene;
mod sdf;
mod vector;

use camera::Camera;
use color::Color;
use geometries::{Plane, Sphere};
use light::PointLight;
use materials::{None, SimpleMaterial};
use point::Point;
use render::Object;
use scene::Scene;
use vector::Vector;

fn main() {
    let camera = Camera {
        origin: Point::ORIGIN,
        azimuth: 0.,
        altitude: 0.,
        width: 1280,
        height: 720,
        fov: 50.,
        min_samples: 15,
        max_samples: 100,
        relative_tolerance: 10e-5,
    };
    let scene = Scene {
        max_bounces: 5,
        objects: vec![
            // Objects
            Object {
                geometry: Box::new(Sphere {
                    center: Point {
                        x: -1.1,
                        y: 0.,
                        z: -2.,
                    },
                    radius: 0.5,
                }),
                material: Box::new(SimpleMaterial {
                    albedo: 0.5,
                    roughness: 1.,
                    color: Color {
                        red: 1.,
                        green: 0.01,
                        blue: 0.01,
                    },
                }),
            },
            Object {
                geometry: Box::new(Sphere {
                    center: Point {
                        x: 0.,
                        y: 0.,
                        z: -2.,
                    },
                    radius: 0.5,
                }),
                material: Box::new(SimpleMaterial {
                    albedo: 0.3,
                    roughness: 1.,
                    color: Color {
                        red: 0.01,
                        green: 1.,
                        blue: 0.01,
                    },
                }),
            },
            Object {
                geometry: Box::new(Sphere {
                    center: Point {
                        x: 0.,
                        y: 0.,
                        z: -4.,
                    },
                    radius: 2.,
                }),
                material: Box::new(SimpleMaterial {
                    albedo: 0.9,
                    roughness: 1.,
                    color: Color {
                        red: 0.01,
                        green: 0.01,
                        blue: 1.,
                    },
                }),
            },
            Object {
                geometry: Box::new(Sphere {
                    center: Point {
                        x: 2.1,
                        y: 0.,
                        z: -2.5,
                    },
                    radius: 0.5,
                }),
                material: Box::new(SimpleMaterial {
                    albedo: 0.8,
                    roughness: 0.2,
                    color: Color {
                        red: 1.,
                        green: 1.,
                        blue: 1.,
                    },
                }),
            },
            Object {
                geometry: Box::new(Plane {
                    origin: Point {
                        x: 0.,
                        y: -2.,
                        z: 0.,
                    },
                    normal: Vector {
                        x: 0.,
                        y: 1.,
                        z: 0.,
                    },
                }),
                material: Box::new(SimpleMaterial {
                    albedo: 0.1,
                    roughness: 1.,
                    color: Color {
                        red: 0.5,
                        green: 0.5,
                        blue: 0.5,
                    },
                }),
            },
        ],
        lights: vec![Box::new(PointLight {
            center: Point {
                x: 2.,
                y: 0.3,
                z: -2.,
            },
            color: Color {
                red: 1.,
                green: 1.,
                blue: 1.,
            },
        })],
        background: Box::new(None {}),
    };

    camera.render(&scene).save("test.png").unwrap();
}
