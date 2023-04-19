use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
use crate::{Vector2, Vector3};

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
    pub fn from_x_yzw(x: f32, yzw: Vector3) -> Vector4 { Vector4::new(x, yzw.x, yzw.y, yzw.z) }
    #[inline]
    pub fn from_xy_zw(xy: Vector2, zw: Vector2) -> Vector4 { Vector4::new(xy.x, xy.y, zw.x, zw.y) }
    #[inline]
    pub fn from_xyz_w(xyz: Vector3, w: f32) -> Vector4 { Vector4::new(xyz.z, xyz.y, xyz.z, w) }

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

    #[inline]
    pub fn as_c_ptr(&self) -> *const f32 { self as *const Vector4 as *const f32 }
    #[inline]
    pub fn as_c_mut_ptr(&mut self) -> *mut f32 { self as *mut Vector4 as *mut f32 }
}

impl Add<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn add(self, rhs: Vector4) -> Vector4 {
        if cfg!(target_feature = "neon") {
            Vector4::add_neon(self, rhs)
        }
        else {
            Vector4::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
        }
    }
}

impl Sub<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn sub(self, rhs: Vector4) -> Vector4 {
        if cfg!(target_feature = "neon") {
            Vector4::sub_neon(self, rhs)
        } else {
            Vector4::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
        }
    }
}

impl Mul<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn mul(self, rhs: Vector4) -> Vector4 {
        if cfg!(target_feature = "neon") {
            Vector4::mul_neon(self, rhs)
        }
        else {
            Vector4::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
        }
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        if cfg!(target_feature = "neon") {
            Vector4::mul_scalar_neon(self, rhs)
        }
        else {
            Vector4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
        }
    }
}

impl Div<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn div(self, rhs: Vector4) -> Self::Output {
        if cfg!(target_feature = "neon") {
            Vector4::div_neon(self, rhs)
        } else {
            Vector4::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.w, )
        }
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        if cfg!(target_feature = "neon") {
            Vector4::div_scalar_neon(self, rhs)
        } else {
            Vector4::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
        }
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

impl From<Vector4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(value: Vector4) -> Self { (value.x, value.y, value.z, value.w) }
}