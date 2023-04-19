use std::arch::aarch64::{vld1q_f32, vmlsq_f32, vmulq_f32, vst1q_f32};
use std::fmt::{Debug, Display, Formatter};
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut, Mul};
use crate::{Vector2, Vector4};


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
    pub fn scale(scale: Vector4) -> Matrix4x4 {
        Matrix4x4 {
            c0: Vector4::new(scale.x, 0., 0., 0.),
            c1: Vector4::new(0., scale.y, 0., 0.),
            c2: Vector4::new(0., 0., scale.z, 0.),
            c3: Vector4::new(0., 0., 0., scale.w)
        }
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
        let c0 = self.c0;
        let c1 = self.c1;
        let c2 = self.c2;
        let c3 = self.c3;

        let c0_0 = c0.x;
        let c0_1 = c0.y;
        let c0_2 = c0.z;
        let c0_3 = c0.w;
        let c1_0 = c1.x;
        let c1_1 = c1.y;
        let c1_2 = c1.z;
        let c1_3 = c1.w;
        let c2_0 = c2.x;
        let c2_1 = c2.y;
        let c2_2 = c2.z;
        let c2_3 = c2.w;
        let c3_0 = c3.x;
        let c3_1 = c3.y;
        let c3_2 = c3.z;
        let c3_3 = c3.w;

        let coef00 = c2_2 * c3_3 - c3_2 * c2_3;
        let coef02 = c1_2 * c3_3 - c3_2 * c1_3;
        let coef03 = c1_2 * c2_3 - c2_2 * c1_3;

        let coef04 = c2_1 * c3_3 - c3_1 * c2_3;
        let coef06 = c1_1 * c3_3 - c3_1 * c1_3;
        let coef07 = c1_1 * c2_3 - c2_1 * c1_3;

        let coef08 = c2_1 * c3_2 - c3_1 * c2_2;
        let coef10 = c1_1 * c3_2 - c3_1 * c1_2;
        let coef11 = c1_1 * c2_2 - c2_1 * c1_2;

        let coef12 = c2_0 * c3_3 - c3_0 * c2_3;
        let coef14 = c1_0 * c3_3 - c3_0 * c1_3;
        let coef15 = c1_0 * c2_3 - c2_0 * c1_3;

        let coef16 = c2_0 * c3_2 - c3_0 * c2_2;
        let coef18 = c1_0 * c3_2 - c3_0 * c1_2;
        let coef19 = c1_0 * c2_2 - c2_0 * c1_2;

        let coef20 = c2_0 * c3_1 - c3_0 * c2_1;
        let coef22 = c1_0 * c3_1 - c3_0 * c1_1;
        let coef23 = c1_0 * c2_1 - c2_0 * c1_1;

        let fac0 = Vector4::new(coef00, coef00, coef02, coef03);
        let fac1 = Vector4::new(coef04, coef04, coef06, coef07);
        let fac2 = Vector4::new(coef08, coef08, coef10, coef11);
        let fac3 = Vector4::new(coef12, coef12, coef14, coef15);
        let fac4 = Vector4::new(coef16, coef16, coef18, coef19);
        let fac5 = Vector4::new(coef20, coef20, coef22, coef23);

        let vec0 = Vector4::new(c1_0, c0_0, c0_0, c0_0);
        let vec1 = Vector4::new(c1_1, c0_1, c0_1, c0_1);
        let vec2 = Vector4::new(c1_2, c0_2, c0_2, c0_2);
        let vec3 = Vector4::new(c1_3, c0_3, c0_3, c0_3);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let sign_a = Vector4::new(1., -1., 1., -1.);
        let sign_b = Vector4::new(-1., 1., -1., 1.);
        let inverse = Matrix4x4::from_columns(
            inv0 * sign_a, inv1 * sign_b, inv2 * sign_a, inv3 * sign_b);

        let row0 = Vector4::new(
            inverse.c0.x, inverse.c1.x, inverse.c2.x, inverse.c3.x);

        let dot0 = c0 * row0;
        let dot1 = (dot0.x + dot0.y) + (dot0.z + dot0.w);

        let one_over_determinant = 1. / dot1;

        inverse * one_over_determinant
    }

    #[inline(always)]
    fn inverse_vec_opt(self: Matrix4x4) -> Matrix4x4 {
        let c0 = self.c0; let c1 = self.c1;
        let c2 = self.c2; let c3 = self.c3;

        let c0_0 = c0.x; let c0_1 = c0.y; let c0_2 = c0.z; let c0_3 = c0.w;
        let c1_0 = c1.x; let c1_1 = c1.y; let c1_2 = c1.z; let c1_3 = c1.w;
        let c2_0 = c2.x; let c2_1 = c2.y; let c2_2 = c2.z; let c2_3 = c2.w;
        let c3_0 = c3.x; let c3_1 = c3.y; let c3_2 = c3.z; let c3_3 = c3.w;

        let coef00_02_03_04_a =
            Vector4::new(c2_2, c1_2, c1_2, c2_1) *
            Vector4::new(c3_3, c3_3, c2_3, c3_3);

        let coef00_02_03_04_b =
            Vector4::new(c3_2, c3_2, c2_2, c3_1) *
            Vector4::new(c2_3, c1_3, c1_3, c2_3);

        let coef06_07_08_10_a =
            Vector4::new(c1_1, c1_1, c2_1, c1_1) *
            Vector4::new(c3_3, c2_3, c3_2, c3_2);

        let coef06_07_08_10_b =
            Vector4::new(c3_1, c2_1, c3_1, c3_1) *
            Vector4::new(c1_3, c1_3, c2_2, c1_2);

        let coef11_12_14_15_a =
            Vector4::new(c1_1, c2_0, c1_0, c1_0) *
            Vector4::new(c2_2, c3_3, c3_3, c2_3);

        let coef11_12_14_15_b =
            Vector4::new(c2_1, c3_0, c3_0, c2_0) *
            Vector4::new(c1_2, c2_3, c1_3, c1_3);

        let coef16_18_19_20_a =
            Vector4::new(c2_0, c1_0, c1_0, c2_0) *
            Vector4::new(c3_2, c3_2, c2_2, c3_1);

        let coef16_18_19_20_b =
            Vector4::new(c3_0, c3_0, c2_0, c3_0) *
            Vector4::new(c2_2, c1_2, c1_2, c2_1);

        let coef00_02_03_04 = coef00_02_03_04_a - coef00_02_03_04_b;
        let coef06_07_08_10 = coef06_07_08_10_a - coef06_07_08_10_b;
        let coef11_12_14_15 = coef11_12_14_15_a - coef11_12_14_15_b;
        let coef16_18_19_20 = coef16_18_19_20_a - coef16_18_19_20_b;

        let coef22 = c1_0 * c3_1 - c3_0 * c1_1;
        let coef23 = c1_0 * c2_1 - c2_0 * c1_1;

        let fac0 = coef00_02_03_04.xxyz();
        let fac1 = Vector4::from_xy_zw(coef00_02_03_04.ww(), coef06_07_08_10.xy());
        let fac2 = Vector4::from_xyz_w(coef06_07_08_10.zzw(), coef11_12_14_15.x);
        let fac3 = coef11_12_14_15.yyzw();
        let fac4 = coef16_18_19_20.xxyz();
        let fac5 = Vector4::from_xy_zw(coef16_18_19_20.ww(), Vector2::new(coef22, coef23));

        let vec0 = Vector4::new(c1_0, c0_0, c0_0, c0_0);
        let vec1 = Vector4::new(c1_1, c0_1, c0_1, c0_1);
        let vec2 = Vector4::new(c1_2, c0_2, c0_2, c0_2);
        let vec3 = Vector4::new(c1_3, c0_3, c0_3, c0_3);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let sign_a = Vector4::new(1., -1., 1., -1.);
        let sign_b = Vector4::new(-1., 1., -1., 1.);
        let inverse = Matrix4x4::from_columns(
            inv0 * sign_a, inv1 * sign_b, inv2 * sign_a, inv3 * sign_b);

        let row0 = Vector4::new(
            inverse.c0.x, inverse.c1.x, inverse.c2.x, inverse.c3.x);

        let dot0 = c0 * row0;
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

    #[inline]
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

