use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
use crate::{Vector3};

#[derive(PartialEq, Copy, Clone)]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 { Vector4 { x, y, z, w } }
    #[inline]
    pub fn all(v: f32) -> Vector4 { Vector4::new(v,v,v,v) }

    #[inline]
    pub fn one() -> Vector4 { Vector4::all(1.) }
    #[inline]
    pub fn zero() -> Vector4 { Vector4::all(0.) }

    #[inline]
    pub fn dot(l: Vector4, r: Vector4) -> f32 {
        l.x * r.x + l.y * r.y + l.z * r.z + l.w * r.w
    }

    #[inline(always)]
    pub fn r(self) -> f32 { self.x }
    #[inline(always)]
    pub fn g(self) -> f32 { self.y }
    #[inline(always)]
    pub fn b(self) -> f32 { self.z }
    #[inline(always)]
    pub fn a(self) -> f32 { self.w }

    #[inline]
    pub fn normalized(self) -> Vector4 { self * (1f32 / Vector4::dot(self, self).sqrt()) }
    #[inline]
    pub fn magnitude(self) -> f32 { Vector4::dot(self, self).sqrt() }
    #[inline]
    pub fn magnitude_sqr(self) -> f32 { Vector4::dot(self, self) }
}

impl Add<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn add(self, rhs: Vector4) -> Vector4 {
        Vector4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn sub(self, rhs: Vector4) -> Vector4 {
        Vector4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Mul<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w,
        )
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vector4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn div(self, rhs: Vector4) -> Self::Output {
        Vector4::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
            self.w / rhs.w,
        )
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Vector4::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl Neg for Vector4 {
    type Output = Vector4;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector4::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Index<usize> for Vector4 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 4);

        unsafe {
            let ptr = self as *const Vector4 as *const f32;
            ptr.add(index).as_ref().unwrap()
        }
    }
}

impl IndexMut<usize> for Vector4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 4);

        unsafe {
            let ptr = self as *mut Vector4 as *mut f32;
            ptr.add(index).as_mut().unwrap()
        }
    }
}

impl Debug for Vector4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector4")
            .field("x", &self[0])
            .field("y", &self[1])
            .field("z", &self[2])
            .field("w", &self[3])
            .finish()
    }
}

impl Display for Vector4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Into<Vector3> for Vector4 {
    fn into(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}