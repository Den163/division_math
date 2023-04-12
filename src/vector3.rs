use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 { Vector3 { x, y, z } }
    #[inline]
    pub fn all(v: f32) -> Vector3 { Vector3 { x: v, y: v, z: v } }

    #[inline]
    pub fn zero() -> Vector3 { Vector3::all(0f32) }
    #[inline]
    pub fn one() -> Vector3 { Vector3::all(1f32) }
    #[inline]
    pub fn forward() -> Vector3 { Vector3 { x: 0f32, y: 0f32, z: 1f32 } }
    #[inline]
    pub fn right() -> Vector3 { Vector3 { x: 1f32, y: 0f32, z: 0f32 } }
    #[inline]
    pub fn up() -> Vector3 { Vector3 { x: 0f32, y: 1f32, z: 0f32 } }

    #[inline]
    pub fn cross(l: Vector3, r: Vector3) -> Vector3 { (l * r.yzx() - l.yzx() * r).yzx() }
    #[inline]
    pub fn dot(l: Vector3, r: Vector3) -> f32 { l.x * r.x + l.y * r.y + l.z * r.z }

    #[inline]
    pub fn zyx(self) -> Vector3 { Vector3::new(self.z, self.y, self.x) }
    #[inline]
    pub fn zxy(self) -> Vector3 { Vector3::new(self.z, self.x, self.y) }
    #[inline]
    pub fn yzx(self) -> Vector3 { Vector3::new(self.y, self.z, self.x) }
    #[inline]
    pub fn yxz(self) -> Vector3 { Vector3::new(self.y, self.x, self.z) }
    #[inline]
    pub fn xzy(self) -> Vector3 { Vector3::new(self.x, self.z, self.y) }
    #[inline]
    pub fn xyz(self) -> Vector3 { Vector3::new(self.x, self.y, self.z) }

    #[inline]
    pub fn magnitude(self) -> f32 { Vector3::dot(self, self).sqrt() }
    #[inline]
    pub fn magnitude_sqr(self) -> f32 { Vector3::dot(self, self) }
}


impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}