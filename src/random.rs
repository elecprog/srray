use std::f32::consts::PI;
use std::f32::consts::TAU;

use rand::random;

use crate::vector::Vector;

// TODO: Use faster and better random number generator
pub fn random_uniform() -> f32 {
    random()
}

pub fn sample_uniform_hemisphere(normal: Vector) -> (Vector, f32) {
    let z = random_uniform();
    let r = (1. - z * z).max(0.).sqrt();
    let phi = TAU * random_uniform();
    let x = r * phi.cos();
    let y = r * phi.sin();

    let (t, s) = normal.orthonormals();
    (x * t + y * s + z * normal, 1. / TAU)
}

pub fn sample_cos_hemisphere(normal: Vector) -> (Vector, f32) {
    let phi = TAU * random_uniform();
    let r = random_uniform().sqrt();

    let x = r * phi.cos();
    let y = r * phi.sin();

    let costheta = (1. - x * x - y * y).max(0.).sqrt();

    let (t, s) = normal.orthonormals();
    (x * t + y * s + costheta * normal, costheta / PI)
}
