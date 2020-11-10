mod camera;
mod color;
mod geometries;
mod lights;
mod materials;
mod point;
mod ray;
mod render;
mod scene;
mod vector;

use camera::Camera;
use color::Color;
use geometries::{Plane, Sphere};
use lights::{DirectionalLight, PointLight};
use materials::{SimpleMaterial, SolidColor};
use point::Point;
use render::Object;
use scene::Scene;
use vector::Vector;

fn main() {
    let camera = Camera {
        origin: Point::ORIGIN,
        azimuth: 0.,
        altitude: 0.,
        width: 1920,
        height: 1080,
        fov: 50.,
        aa_factor: 2,
    };
    let scene = Scene {
        max_bounces: 20,
        objects: vec![
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
                        z: -2.,
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
        ],
        lights: vec![
            &DirectionalLight {
                direction: Vector {
                    x: 0.5,
                    y: -1.,
                    z: -0.5,
                },
                intensity: 3.,
                color: Color {
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                },
            },
            &PointLight {
                position: Point {
                    x: 2.,
                    y: 0.3,
                    z: -2.,
                },
                intensity: 30.,
                color: Color {
                    red: 1.,
                    green: 0.5,
                    blue: 1.,
                },
            },
        ],
        background: &SolidColor {
            color: Color {
                red: 0.2,
                green: 0.4,
                blue: 0.5,
            },
        },
    };

    camera.render(&scene).save("test.png").unwrap();
}
