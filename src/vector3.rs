use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
use std::fmt::Debug;
use crate::Vector4;

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
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
    pub fn forward() -> Vector3 { Vector3 { x: 0., y: 0., z: 1. } }
    #[inline]
    pub fn right() -> Vector3 { Vector3 { x: 1., y: 0., z: 0. } }
    #[inline]
    pub fn up() -> Vector3 { Vector3 { x: 0., y: 1., z: 0. } }

    #[inline]
    pub fn cross(l: Vector3, r: Vector3) -> Vector3 {
        let c = l * Vector3::new(r.y,r.z,r.x) -
                         Vector3::new(l.y,l.z,l.x) * r;
        Vector3::new(c.y, c.z, c.x)
    }
    #[inline]
    pub fn dot(l: Vector3, r: Vector3) -> f32 { l.x * r.x + l.y * r.y + l.z * r.z }

    #[inline(always)]
    pub fn r(self) -> f32 { self.x }
    #[inline(always)]
    pub fn g(self) -> f32 { self.y }
    #[inline(always)]
    pub fn b(self) -> f32 { self.z }

    #[inline]
    pub fn normalized(self) -> Vector3 { self * (1f32 / Vector3::dot(self, self).sqrt()) }
    #[inline]
    pub fn magnitude(self) -> f32 { Vector3::dot(self, self).sqrt() }
    #[inline]
    pub fn magnitude_sqr(self) -> f32 { Vector3::dot(self, self) }

    #[inline]
    pub fn to_vec4_as_point(self) -> Vector4 {
        Vector4::new(self.x, self.y, self.z, 1.)
    }

    #[inline]
    pub fn to_vec4_as_direction(self) -> Vector4 {
        Vector4::new(self.x, self.y, self.z, 0.)
    }
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
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
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

impl Index<usize> for Vector3 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 3);

        unsafe {
            let ptr = self as *const Vector3 as *const f32;
            & *ptr.add(index)
        }
    }
}

impl IndexMut<usize> for Vector3 {

    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 3);

        unsafe {
            let ptr = self as *mut Vector3 as *mut f32;
            &mut *ptr.add(index)
        }
    }
}