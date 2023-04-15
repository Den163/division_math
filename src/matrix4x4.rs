use std::ops::{Index, IndexMut, Mul};
use crate::Vector4;

#[repr(C)]
pub struct Matrix4x4 {
    columns: [Vector4; 4],
}

impl Matrix4x4 {
    pub fn from_columns(columns: [Vector4; 4]) -> Matrix4x4 { Matrix4x4 { columns } }
    pub fn all(v: f32) -> Matrix4x4 { Matrix4x4 { columns: [Vector4::all(v); 4] } }

    pub fn identity() -> Self {
        Matrix4x4 {
            columns: [
                Vector4::new(1., 0., 0., 0.),
                Vector4::new(0., 1., 0., 0.),
                Vector4::new(0., 0., 1., 0.),
                Vector4::new(0., 0., 0., 1.)
            ]
        }
    }

    pub fn zero() -> Self { Matrix4x4 { columns: [Vector4::zero(); 4] } }

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

    fn index(&self, index: usize) -> &Vector4 { &self.columns[index] }
}

impl IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.columns[index] }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4x4::from_columns(
            [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2], self[3] * rhs[3]]
        )
    }
}