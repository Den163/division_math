use std::ops::{Add, Mul, Sub, Div};

use crate::{Matrix4x4, Vector2, Vector3, Vector4};

pub const EPSILON_SQR: f32 = (f32::EPSILON as f64 * f32::EPSILON as f64) as f32;

pub fn lerp<TV, TL>(p0: TV, p1: TV, t: TL) -> TV
where
    TV: Add<Output = TV> + Mul<TL, Output = TV> + Sub<Output = TV> + Copy,
{
    p0 + (p1 - p0) * t
}

pub fn inverse_lerp<T>(p0: T, p1: T, v: T) -> T 
where T: Div<Output = T> + Sub<Output = T> + Copy {
    (v - p0) / (p1 - p0)
}

pub fn approx(x: f32, y: f32) -> bool {
    let d = y - x;
    d * d <= EPSILON_SQR
}

include!(concat!(env!("OUT_DIR"), "/vector.math.gen.rs"));

impl Matrix4x4 {
    pub fn approx(x: Matrix4x4, y: Matrix4x4) -> bool {
        Vector4::approx(x[0], y[0])
            && Vector4::approx(x[1], y[1])
            && Vector4::approx(x[2], y[2])
            && Vector4::approx(x[3], y[3])
    }
}
