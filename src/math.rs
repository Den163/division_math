use crate::{Vector2, Vector3, Vector4};

pub const EPSILON_SQR: f32 = (f32::EPSILON as f64 * f32::EPSILON as f64) as f32;

pub fn lerp(p0: f32, p1: f32, t: f32) -> f32 {  p0 + t * (p1 - p0) }

pub fn approx(x: f32, y: f32) -> bool {
    let d = y - x;
    d * d <= EPSILON_SQR
}

include!(concat!(env!("OUT_DIR"), "/vector.math.gen.rs"));