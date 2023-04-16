use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
use std::fmt::{Debug, Display, Formatter};
use crate::{Vector4};

#[derive(PartialEq, Copy, Clone)]
#[repr(C)]
pub struct Vector3 {
    pub xyz: [f32; 3],
}

impl Vector3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 { Vector3 { xyz: [x, y, z] } }
    #[inline]
    pub fn all(v: f32) -> Vector3 { Vector3 { xyz: [v, v, v] } }

    #[inline]
    pub fn zero() -> Vector3 { Vector3::all(0f32) }
    #[inline]
    pub fn one() -> Vector3 { Vector3::all(1f32) }
    #[inline]
    pub fn forward() -> Vector3 { Vector3 { xyz: [0f32, 0f32, 1f32] } }
    #[inline]
    pub fn right() -> Vector3 { Vector3 { xyz: [1f32, 0f32, 0f32] } }
    #[inline]
    pub fn up() -> Vector3 { Vector3 { xyz: [0f32, 1f32, 0f32] } }

    #[inline]
    pub fn cross(l: Vector3, r: Vector3) -> Vector3 { (l * r.yzx() - l.yzx() * r).yzx() }
    #[inline]
    pub fn dot(l: Vector3, r: Vector3) -> f32 { l.x() * r.x() + l.y() * r.y() + l.z() * r.z() }

    #[inline(always)]
    pub fn x(&self) -> f32 { self.xyz[0] }
    #[inline(always)]
    pub fn y(&self) -> f32 { self.xyz[1] }
    #[inline(always)]
    pub fn z(&self) -> f32 { self.xyz[2] }

    #[inline(always)]
    pub fn r(&self) -> f32 { self.x() }
    #[inline(always)]
    pub fn g(&self) -> f32 { self.y() }
    #[inline(always)]
    pub fn b(&self) -> f32 { self.z() }

    #[inline]
    pub fn xyz(&self) -> Vector3 { Vector3::new(self.x(), self.y(), self.z()) }
    #[inline]
    pub fn xzy(&self) -> Vector3 { Vector3::new(self.x(), self.z(), self.y()) }
    #[inline]
    pub fn yxz(&self) -> Vector3 { Vector3::new(self.y(), self.x(), self.z()) }
    #[inline]
    pub fn yzx(&self) -> Vector3 { Vector3::new(self.y(), self.z(), self.x()) }
    #[inline]
    pub fn zxy(&self) -> Vector3 { Vector3::new(self.z(), self.x(), self.y()) }
    #[inline]
    pub fn zyx(&self) -> Vector3 { Vector3::new(self.z(), self.y(), self.x()) }

    #[inline]
    pub fn normalized(self) -> Vector3 { self * (1f32 / Vector3::dot(self, self).sqrt()) }
    #[inline]
    pub fn magnitude(self) -> f32 { Vector3::dot(self, self).sqrt() }
    #[inline]
    pub fn magnitude_sqr(self) -> f32 { Vector3::dot(self, self) }

    #[inline]
    pub fn to_vec4_as_point(self) -> Vector4 {
        Vector4::new(self.x(), self.y(), self.z(), 1.)
    }

    #[inline]
    pub fn to_vec4_as_direction(self) -> Vector4 {
        Vector4::new(self.x(), self.y(), self.z(), 0.)
    }
}


impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 3);

        &self.xyz[index]
    }
}

impl IndexMut<usize> for Vector3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 3);

        &mut self.xyz[index]
    }
}

impl Debug for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector3")
            .field("x", &self.xyz[0])
            .field("y", &self.xyz[1])
            .field("z", &self.xyz[2])
            .finish()
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}