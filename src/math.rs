use crate::{Matrix4x4, Vector2, Vector3, Vector4};

pub const EPSILON_SQR: f32 = (f32::EPSILON as f64 * f32::EPSILON as f64) as f32;

pub fn lerp(p0: f32, p1: f32, t: f32) -> f32 {  p0 + t * (p1 - p0) }

pub fn approx(x: f32, y: f32) -> bool {
    let d = y - x;
    d * d <= EPSILON_SQR
}

include!(concat!(env!("OUT_DIR"), "/vector.math.gen.rs"));

impl Matrix4x4 {
    pub fn approx(x: Matrix4x4, y: Matrix4x4) -> bool {
        Vector4::approx(x[0], y[0]) &&
        Vector4::approx(x[1], y[1]) &&
        Vector4::approx(x[2], y[2]) &&
        Vector4::approx(x[3], y[3])
    }
}