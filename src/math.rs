pub const EPSILON_64: f64 = 2.22044604925031308085e-16;
pub const EPSILON_32: f32 = 1.1920928955078125e-7f32;

pub fn lerp(p0: f32, p1: f32, t: f32) -> f32 {  p0 + t * (p1 - p0) }