use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut, Mul};
use crate::Vector4;

#[derive(PartialEq, Copy, Clone)]
#[repr(C)]
pub struct Matrix4x4 {
    c0: Vector4,
    c1: Vector4,
    c2: Vector4,
    c3: Vector4,
}

impl Matrix4x4 {
    #[inline]
    pub fn from_columns(c0: Vector4, c1: Vector4, c2: Vector4, c3: Vector4) -> Matrix4x4 {
        Matrix4x4 { c0, c1, c2, c3 }
    }

    #[inline]
    pub fn all(v: f32) -> Matrix4x4 {
        let v = Vector4::all(v);
        Matrix4x4::from_columns(v, v, v, v)
    }

    #[inline]
    pub fn identity() -> Matrix4x4 {
        Matrix4x4::from_columns(
            Vector4::new(1., 0., 0., 0.),
            Vector4::new(0., 1., 0., 0.),
            Vector4::new(0., 0., 1., 0.),
            Vector4::new(0., 0., 0., 1.),
        )
    }

    #[inline]
    pub fn zero() -> Matrix4x4 { Matrix4x4::all(0.) }

    #[inline]
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Matrix4x4::from_columns(
            Vector4::new(2. / (right - left), 0., 0., 0.),
            Vector4::new(0., 2. / (top - bottom), 0., 0.),
            Vector4::new(0., 0., -2. / (far - near), 0.),
            Vector4::new(
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                -(far + near) / (far - near),
                1.,
            ),
        )
    }

    pub fn inverse(self) -> Matrix4x4 {
        let coef00 = self[2][2] * self[3][3] - self[3][2] * self[2][3];
        let coef02 = self[1][2] * self[3][3] - self[3][2] * self[1][3];
        let coef03 = self[1][2] * self[2][3] - self[2][2] * self[1][3];

        let coef04 = self[2][1] * self[3][3] - self[3][1] * self[2][3];
        let coef06 = self[1][1] * self[3][3] - self[3][1] * self[1][3];
        let coef07 = self[1][1] * self[2][3] - self[2][1] * self[1][3];

        let coef08 = self[2][1] * self[3][2] - self[3][1] * self[2][2];
        let coef10 = self[1][1] * self[3][2] - self[3][1] * self[1][2];
        let coef11 = self[1][1] * self[2][2] - self[2][1] * self[1][2];

        let coef12 = self[2][0] * self[3][3] - self[3][0] * self[2][3];
        let coef14 = self[1][0] * self[3][3] - self[3][0] * self[1][3];
        let coef15 = self[1][0] * self[2][3] - self[2][0] * self[1][3];

        let coef16 = self[2][0] * self[3][2] - self[3][0] * self[2][2];
        let coef18 = self[1][0] * self[3][2] - self[3][0] * self[1][2];
        let coef19 = self[1][0] * self[2][2] - self[2][0] * self[1][2];

        let coef20 = self[2][0] * self[3][1] - self[3][0] * self[2][1];
        let coef22 = self[1][0] * self[3][1] - self[3][0] * self[1][1];
        let coef23 = self[1][0] * self[2][1] - self[2][0] * self[1][1];

        let fac0 = Vector4::new(coef00, coef00, coef02, coef03);
        let fac1 = Vector4::new(coef04, coef04, coef06, coef07);
        let fac2 = Vector4::new(coef08, coef08, coef10, coef11);
        let fac3 = Vector4::new(coef12, coef12, coef14, coef15);
        let fac4 = Vector4::new(coef16, coef16, coef18, coef19);
        let fac5 = Vector4::new(coef20, coef20, coef22, coef23);

        let vec0 = Vector4::new(self[1][0], self[0][0], self[0][0], self[0][0]);
        let vec1 = Vector4::new(self[1][1], self[0][1], self[0][1], self[0][1]);
        let vec2 = Vector4::new(self[1][2], self[0][2], self[0][2], self[0][2]);
        let vec3 = Vector4::new(self[1][3], self[0][3], self[0][3], self[0][3]);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let sign_a = Vector4::new(1., -1., 1., -1.);
        let sign_b = Vector4::new(-1., 1., -1., 1.);
        let inverse = Matrix4x4::from_columns(
            inv0 * sign_a, inv1 * sign_b, inv2 * sign_a, inv3 * sign_b);

        let row0 = Vector4::new(
            inverse[0][0], inverse[1][0], inverse[2][0], inverse[3][0]);

        let dot0 = self[0] * row0;
        let dot1 = (dot0.x + dot0.y) + (dot0.z + dot0.w);

        let one_over_determinant = 1. / dot1;

        inverse * one_over_determinant
    }
}

impl Index<usize> for Matrix4x4 {
    type Output = Vector4;

    #[inline]
    fn index(&self, index: usize) -> &Vector4 {
        assert!(index < 4);

        unsafe {
            let ptr = self as *const Matrix4x4 as *const Vector4;
            ptr.add(index).as_ref().unwrap()
        }
    }
}

impl IndexMut<usize> for Matrix4x4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 4);

        unsafe {
            let ptr = self as *mut Matrix4x4 as *mut Vector4;
            ptr.add(index).as_mut().unwrap()
        }
    }
}

impl Mul<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Matrix4x4::from_columns(
            self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs,
        )
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        Matrix4x4::from_columns(
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2],
            self[3] * rhs[3],
        )
    }
}

impl Debug for Matrix4x4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix4x4")
            .field("c0: ", &self[0])
            .field("c1: ", &self[1])
            .field("c2: ", &self[2])
            .field("c3: ", &self[3])
            .finish()
    }
}

impl Display for Matrix4x4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

