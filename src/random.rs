use std::f32::consts::PI;
use std::f32::consts::TAU;

use std::f32;

use crate::vector::Vector;

static mut RANDOM_STATE: u64 = 749738427937290380;

pub fn next_random_state() -> u64 {
    unsafe {
        /* XORSHIFT */
        let mut s = RANDOM_STATE;
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        RANDOM_STATE = s;
        s
    }
}

pub fn random_uniform() -> f32 {
    let r = next_random_state();

    const MANTISSA_BITS: u32 = 23;
    const EXP_BIAS: u32 = 127;

    /* r is 64 bits */
    let m = (r >> (63 - MANTISSA_BITS)) as u32;
    let e = EXP_BIAS << MANTISSA_BITS;
    f32::from_bits(m | e) - 1.
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
