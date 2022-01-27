mod camera;
mod color;
mod geometries;
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
use materials::{DiffuseEmitter, SimpleMaterial};
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
        min_samples: 25,
        max_samples: 100,
        relative_tolerance: 10e-5,
    };
    let scene = Scene {
        max_bounces: 5,
        objects: vec![
            // Objects
            &Object {
                geometry: &Sphere {
                    center: Point {
                        x: -1.1,
                        y: 0.,
                        z: -2.,
                    },
                    radius: 0.5,
                },
                material: &SimpleMaterial {
                    albedo: 0.5,
                    roughness: 1.,
                    color: Color {
                        red: 1.,
                        green: 0.,
                        blue: 0.,
                    },
                },
            },
            &Object {
                geometry: &Sphere {
                    center: Point {
                        x: 0.,
                        y: 0.,
                        z: -2.,
                    },
                    radius: 0.5,
                },
                material: &SimpleMaterial {
                    albedo: 0.3,
                    roughness: 1.,
                    color: Color {
                        red: 0.,
                        green: 1.,
                        blue: 0.,
                    },
                },
            },
            &Object {
                geometry: &Sphere {
                    center: Point {
                        x: 0.,
                        y: 0.,
                        z: -4.,
                    },
                    radius: 2.,
                },
                material: &SimpleMaterial {
                    albedo: 0.8,
                    roughness: 1.,
                    color: Color {
                        red: 0.,
                        green: 0.,
                        blue: 1.,
                    },
                },
            },
            &Object {
                geometry: &Sphere {
                    center: Point {
                        x: 2.1,
                        y: 0.,
                        z: -2.5,
                    },
                    radius: 0.5,
                },
                material: &SimpleMaterial {
                    albedo: 0.8,
                    roughness: 0.2,
                    color: Color {
                        red: 1.,
                        green: 1.,
                        blue: 1.,
                    },
                },
            },
            &Object {
                geometry: &Plane {
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
                },
                material: &SimpleMaterial {
                    albedo: 0.1,
                    roughness: 1.,
                    color: Color {
                        red: 0.5,
                        green: 0.5,
                        blue: 0.5,
                    },
                },
            },
            // Lights
            &Object {
                geometry: &Sphere {
                    radius: 0.1,
                    center: Point {
                        x: 2.,
                        y: 0.3,
                        z: -2.,
                    },
                },
                material: &DiffuseEmitter {
                    color: Color {
                        red: 5.,
                        green: 5.,
                        blue: 5.,
                    },
                },
            },
        ],
        background: &DiffuseEmitter {
            color: Color {
                red: 0.3,
                green: 0.4,
                blue: 0.6,
            },
        },
    };

    camera.render(&scene).save("test.png").unwrap();
}
