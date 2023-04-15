use std::ops::{Index, IndexMut, Mul};
use crate::Vector4;

#[repr(C)]
pub struct Matrix4x4 {
    columns: [Vector4; 4],
}

impl Matrix4x4 {
    #[inline]
    pub fn from_columns(columns: [Vector4; 4]) -> Matrix4x4 { Matrix4x4 { columns } }

    #[inline]
    pub fn all(v: f32) -> Matrix4x4 { Matrix4x4 { columns: [Vector4::all(v); 4] } }

    #[inline]
    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            columns: [
                Vector4::new(1., 0., 0., 0.),
                Vector4::new(0., 1., 0., 0.),
                Vector4::new(0., 0., 1., 0.),
                Vector4::new(0., 0., 0., 1.)
            ]
        }
    }

    #[inline]
    pub fn zero() -> Matrix4x4 { Matrix4x4 { columns: [Vector4::zero(); 4] } }

    #[inline]
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Matrix4x4 {
            columns: [
                Vector4::new(2. / (right - left), 0., 0., 0.),
                Vector4::new(0., 2. / (top - bottom), 0., 0.),
                Vector4::new(0., 0., -2. / (far - near), 0.),
                Vector4::new(
                    -(right + left) / (right - left),
                    -(top + bottom) / (top - bottom),
                    -(far + near) / (far - near),
                    1.,
                )
            ]
        }
    }
}

impl Index<usize> for Matrix4x4 {
    type Output = Vector4;

    #[inline]
    fn index(&self, index: usize) -> &Vector4 { &self.columns[index] }
}

impl IndexMut<usize> for Matrix4x4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.columns[index] }
}

impl Mul<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Matrix4x4::from_columns([
            self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs
        ])
    }
}