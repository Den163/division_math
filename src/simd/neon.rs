use std::arch::aarch64::{float32x4_t, vaddq_f32, vdivq_f32, vld1q_f32, vmulq_f32, vmulq_n_f32, vst1q_f32, vsubq_f32};
use std::mem::MaybeUninit;
use crate::Vector4;

#[cfg(target_feature = "neon")]
impl Vector4 {
    #[inline(always)]
    pub(crate) fn mul_simd(lhs: Vector4, rhs: Vector4) -> Vector4 {
        unsafe {
            let x = lhs.load_to_neon();
            let y = rhs.load_to_neon();
            let r = vmulq_f32(x, y);

            Vector4::from_neon_ptr(r)
        }
    }

    #[inline(always)]
    pub(crate) fn mul_scalar_simd(lhs: Vector4, rhs: f32) -> Vector4 {
        unsafe {
            let x = lhs.load_to_neon();
            let r = vmulq_n_f32(x, rhs);

            Vector4::from_neon_ptr(r)
        }
    }

    #[inline(always)]
    pub(crate) fn div_simd(lhs: Vector4, rhs: Vector4) -> Vector4 {
        unsafe {
            let x = lhs.load_to_neon();
            let y = rhs.load_to_neon();
            let r = vdivq_f32(x, y);

            Vector4::from_neon_ptr(r)
        }
    }

    #[inline(always)]
    pub(crate) fn div_scalar_simd(lhs: Vector4, rhs: f32) -> Vector4 {
        Vector4::div_simd(lhs, Vector4::all(rhs))
    }

    #[inline(always)]
    pub(crate) fn add_simd(lhs: Vector4, rhs: Vector4) -> Vector4 {
        unsafe {
            let x = lhs.load_to_neon();
            let y = rhs.load_to_neon();
            let r = vaddq_f32(x, y);

            Vector4::from_neon_ptr(r)
        }
    }

    #[inline(always)]
    pub(crate) fn sub_simd(lhs: Vector4, rhs: Vector4) -> Vector4 {
        unsafe {
            let x = lhs.load_to_neon();
            let y = rhs.load_to_neon();
            let r = vsubq_f32(x, y);

            Vector4::from_neon_ptr(r)
        }
    }

    #[inline(always)]
    fn from_neon_ptr(neon_ptr: float32x4_t) -> Vector4 {
        unsafe {
            let mut result = MaybeUninit::<Vector4>::uninit();
            vst1q_f32(result.as_mut_ptr() as *mut f32, neon_ptr);
            *result.as_ptr()
        }
    }

    #[inline(always)]
    fn load_to_neon(&self) -> float32x4_t {
        unsafe {
            vld1q_f32(self.as_ptr())
        }
    }
}