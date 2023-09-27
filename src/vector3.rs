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
    pub fn to_vec4_as_point(self) -> Vector4 {
        Vector4::new(self.x, self.y, self.z, 1.)
    }

    #[inline]
    pub fn to_vec4_as_direction(self) -> Vector4 {
        Vector4::new(self.x, self.y, self.z, 0.)
    }
}